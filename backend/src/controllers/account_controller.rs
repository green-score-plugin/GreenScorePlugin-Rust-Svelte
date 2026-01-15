use axum::extract::State;
use axum::Json;
use sqlx::MySqlPool;
use serde::Deserialize;
use serde_json::{json, Value};
use tower_sessions::Session;
use crate::models::{Account, User};
use bcrypt;
use sqlx::Row;



#[derive(Deserialize)]
pub struct UpdateAccountRequest {
    #[serde(default)]
    pub email: Option<String>,

    #[serde(default)]
    pub prenom: Option<String>,

    #[serde(default)]
    pub nom: Option<String>,

    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct JoinOrgaRequest {
    pub code: String,
}

pub async fn update_account(
    session: Session,
    State(pool): State<MySqlPool>,
    Json(payload): Json<UpdateAccountRequest>,
) -> Json<Value> {
    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    let mut user = match account_opt {
        Some(Account::User(u)) => u,
        _ => {
            return Json(json!({
                "success": false,
                "message": "Non authentifié"
            }));
        }
    };

    let mut query = String::from("UPDATE user SET ");
    let mut updates = Vec::new();
    let mut params: Vec<String> = Vec::new();

    if let Some(ref email) = payload.email {
        updates.push("email = ?");
        params.push(email.clone());
        user.email = email.clone();
    }

    if let Some(ref prenom) = payload.prenom {
        updates.push("first_name = ?");
        params.push(prenom.clone());
        user.prenom = prenom.clone();
    }

    if let Some(ref nom) = payload.nom {
        updates.push("last_name = ?");
        params.push(nom.clone());
        user.nom = nom.clone();
    }

    if let Some(ref password) = payload.password {
        let hash = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
            Ok(h) => h,
            Err(e) => {
                return Json(json!({
                    "success": false,
                    "message": format!("Erreur hash mot de passe: {}", e)
                }));
            }
        };
        updates.push("password = ?");
        params.push(hash);
    }

    if updates.is_empty() {
        return Json(json!({
            "success": false,
            "message": "Aucune modification à effectuer"
        }));
    }

    query.push_str(&updates.join(", "));
    query.push_str(" WHERE id = ?");

    let mut q = sqlx::query(&query);
    for param in params {
        q = q.bind(param);
    }
    q = q.bind(user.id);

    if let Err(e) = q.execute(&pool).await {
        return Json(json!({
            "success": false,
            "message": format!("Erreur mise à jour: {}", e)
        }));
    }

    let updated_account = Account::User(User {
        id: user.id,
        email: user.email.clone(),
        prenom: user.prenom.clone(),
        nom: user.nom.clone(),
    });

    session.insert("account", updated_account.clone()).await.unwrap();

    Json(json!({
                        "success": true,
                        "account": updated_account
                    }))

}


pub async fn delete_account( session: Session, State(pool): State<MySqlPool>) -> Json<Value> {

    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    let account = match account_opt {
        Some(acc) => acc,
        None => return Json(json!({"success": false, "message": "Non authentifié"})),
    };

    let id_to_delete = match &account {
        Account::User(u) => u.id,
        Account::Organisation(o) => o.admin_id,
    };

    match sqlx::query("DELETE FROM user WHERE id = ?")
        .bind(id_to_delete)
        .execute(&pool)
        .await
    {
        Ok(_) => {
            session.delete().await.unwrap();
            Json(json!({"success": true}))
        }
        Err(e) => Json(json!({"success": false, "message": format!("Erreur suppression compte: {}", e)})),
    }

}


pub async fn join_organization(
    session: Session,
    State(pool): State<MySqlPool>,
    Json(payload): Json<JoinOrgaRequest>,
) -> Json<Value> {
    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    let user = match account_opt {
        Some(Account::User(u)) => u,
        _ => return Json(json!({ "success": false, "message": "Non authentifié" })),
    };


    let row_opt = match sqlx::query("SELECT id FROM organisation WHERE organisation_code = ?")
        .bind(&payload.code)
        .fetch_optional(&pool)
        .await
    {
        Ok(r) => r,
        Err(e) => return Json(json!({ "success": false, "message": format!("Erreur technique recherche: {}", e) }))
    };

    let org_id: i32 = match row_opt {
        Some(row) => row.try_get("id").unwrap_or(0),
        None => return Json(json!({ "success": false, "message": format!("Code invalide: '{}'. Aucune organisation trouvée.", payload.code) }))
    };

    if let Err(e) = sqlx::query("UPDATE user SET organisation_id = ? WHERE id = ?")
        .bind(org_id)
        .bind(user.id)
        .execute(&pool)
        .await
    {
        return Json(json!({ "success": false, "message": format!("Erreur jonction: {}", e) }));
    }


    Json(json!({
        "success": true,
        "message": "Organisation rejointe avec succès"
    }))
}

pub async fn get_organisation_member(session: Session, State(pool): State<MySqlPool>) -> Json<Value> {
    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    let organisation = match account_opt {
        Some(Account::Organisation(o)) => o,
        _ => return Json(json!({ "success": false, "message": "Non authentifié en tant qu'organisation" })),
    };

    let members = match sqlx::query_as::<_, User>(
        "SELECT id, email, first_name AS prenom, last_name AS nom FROM user WHERE organisation_id = ?"
    )
        .bind(organisation.id)
        .fetch_all(&pool)
        .await
    {
        Ok(r) => r,
        Err(e) => return Json(json!({ "success": false, "message": format!("Erreur récupération membres: {}", e) })),
    };

    Json(json!({
        "success": true,
        "members": members
    }))
}

pub async fn remove_organisation_member(State(pool): State<MySqlPool>, Json(payload) : Json<Value>) -> Json<Value> {

    match sqlx::query("UPDATE user SET organisation_id = NULL WHERE id = ?")
        .bind(payload["userId"].as_str().unwrap())
        .execute(&pool)
        .await
    {
        Ok(_) => Json(json!({ "success": true })),
        Err(e) => Json(json!({ "success": false, "message": format!("Erreur suppression membre: {}", e) })),
    }

}