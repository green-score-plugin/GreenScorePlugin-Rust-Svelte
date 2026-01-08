use axum::extract::State;
use axum::Json;
use sqlx::MySqlPool;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
) -> Json<serde_json::Value> {
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

    return Json(json!({
                        "success": true,
                        "account": updated_account
                    }));

}


pub async fn delete_account(
    session: Session,
    State(pool): State<MySqlPool>,
) -> Json<serde_json::Value> {
    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    let user = match account_opt {
        Some(Account::User(u)) => u,
        _ => {
            return Json(json!({
                "success": false,
                "message": "Non authentifié"
            }));
        }
    };

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("Erreur de connexion BDD: {}", e)
        }))
    };

    if let Err(e) = sqlx::query("DELETE FROM monitored_website WHERE user_id = ?")
        .bind(user.id)
        .execute(&mut *tx)
        .await
    {
        return Json(json!({
            "success": false,
            "message": format!("Erreur suppression sites web: {}", e)
        }));
    }

    if let Err(e) = sqlx::query("DELETE FROM user WHERE id = ?")
        .bind(user.id)
        .execute(&mut *tx)
        .await
    {
        return Json(json!({
            "success": false,
            "message": format!("Erreur suppression compte: {}", e)
        }));
    }

    if let Err(e) = tx.commit().await {
        return Json(json!({
            "success": false,
            "message": format!("Erreur validation transaction: {}", e)
        }));
    }

    return Json(json!({
        "success": true,
    }));
}






pub async fn join_organization(
    session: Session,
    State(pool): State<MySqlPool>,
    Json(payload): Json<JoinOrgaRequest>,
) -> Json<serde_json::Value> {
    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    let mut user = match account_opt {
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


    return Json(json!({
        "success": true,
        "message": "Organisation rejointe avec succès"
    }));
}
