use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;
use tower_sessions::Session;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::models::organisation::Organisation;
use crate::dto::user_full::UserFull;
use crate::service::auth_service::AuthService;

#[derive(Deserialize)]
pub struct InscriptionRequest {
    pub email: String,
    pub password: String,
    pub lastname: String,
    pub firstname: String,
}

#[derive(Deserialize)]
pub struct InscriptionOrgaRequest {
    pub orga_name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siret : Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}

pub async fn login(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<LoginRequest>) -> Json<Value> {

    let row = match sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, email, CAST(password AS CHAR) as password, CAST(roles AS CHAR) as roles  FROM user WHERE email = ?",
    )
        .bind(payload["email"].as_str().unwrap_or(""))
        .fetch_optional(&pool)
        .await {
        Ok(u) => u,
        Err(_) => return Json(json!({
        "success": false,
        "message": "errors.auth.connection_error"
        })),
    };

    if row.is_none() {
        return Json(json!({
            "success": false,
            "message": "errors.auth.invalid_credentials",
        }));
    }

    let (user_id, email, password_hash, roles) = row.unwrap();
    let password = payload["password"].as_str().unwrap_or("");
    if !bcrypt::verify(password, &password_hash).unwrap_or(false) {
        Json(json!({
            "success": false,
            "message": "errors.auth.invalid_credentials",
        }))
    } else {

        if roles == "[\"ROLE_USER\"]" {
            match sqlx::query_as::<_, (String, String, Option<i64>)>(
                "SELECT first_name, last_name, organisation_id FROM user WHERE email = ?"
            )
                .bind(&email)
                .fetch_optional(&pool)
                .await {
                Ok(Some((first_name, last_name, organisation_id))) => {
                    let account = Account::User(User { id: user_id, email, prenom: first_name, nom: last_name, id_orga: organisation_id});
                    session.insert("account", account.clone()).await.unwrap();

                    return Json(json!({
                        "success": true,
                        "account": account
                    }));
                },
                Ok(None) => {

                },
                Err(_) => {
                    return Json(json!({
                "success": false,
                "message": "errors.auth.user_retrieval_error"
            }));
                }
            }
        }

        if roles == "[\"ROLE_ORGANISATION\"]" {
            match sqlx::query_as::<_, (i64, String, String, Option<String>)>(
                "SELECT o.id, o.organisation_name, o.organisation_code, o.siret FROM user u
             JOIN organisation o ON o.admin_id = u.id
             WHERE email = ?"
            )
                .bind(&email)
                .fetch_optional(&pool)
                .await {
                Ok(Some((organisation_id, organisation_name, organisation_code, siret))) => {
                    let account = Account::Organisation(Organisation {
                        id: organisation_id,
                        nom: organisation_name,
                        siret,
                        code: organisation_code,
                        admin_id: user_id
                    });
                    session.insert("account", account).await.unwrap();

                    return Json(json!({
                    "success" : true
                }))
                },
                Ok(None) => {

                },
                Err(_) => {
                    return Json(json!({
                "success": false,
                "message": "errors.auth.connection_error"
            }));
                }
            }
        }

        Json(json!({
            "success": false,
            "message": "errors.auth.invalid_credentials"
        }))
    }
}

pub async fn inscription(session: Session, pool: &MySqlPool, Json(payload): Json<InscriptionRequest>) -> Json<GenericResponse> {
    let email = payload.email.trim();
    let password = payload.password.trim();
    let first_name = payload.firstname.trim();
    let last_name = payload.lastname.trim();

    if email.is_empty() || password.is_empty() || first_name.is_empty() || last_name.is_empty() {
        return Json(GenericResponse {
            success: false,
            message: Some("errors.auth.missing_fields".to_string()),
        });
    }

    let user_id = match AuthService::inscription(pool, email, password, first_name, last_name).await {
        Ok(id) => id,
        Err(msg) => return Json(GenericResponse { success: false, message: Some(msg) }),
    };

    let user = User {
        id: user_id,
        id_organisation: None,
        id_service: None,
        email: email.to_string(),
        prenom: first_name.to_string(),
        nom: last_name.to_string(),
        est_admin: false,
        total_carbon_footprint: 0.0,
    };

    let user_full = UserFull {
        user: user.clone(),
        organisation: None,
        service: None,
    };

    session.insert("userFull", user_full).await.unwrap();

    Json(GenericResponse {
        success: true,
        message: None,
    })

}

pub async fn inscription_orga(session: Session, pool: &MySqlPool, Json(payload): Json<InscriptionOrgaRequest>) -> Json<GenericResponse>
{

    let orga_name = payload.orga_name.trim();
    let siret = payload.siret.as_ref().map(|s| s.trim().to_string());


    let mut user_full = session.get::<UserFull>("userFull").await.unwrap().unwrap();

    let user_id = user_full.user.id;

    let result = AuthService::inscription_orga(pool, orga_name, siret.as_deref(), user_id).await;

    let (organisation_id, organisation_code) = match result {
        Ok(tuple) => tuple,
        Err(msg) => return Json(GenericResponse { success: false, message: Some(msg) }),
    };

    let organisation = Organisation {
        id: organisation_id,
        nom: orga_name.to_string(),
        siret,
        code: organisation_code,
    };

    user_full.organisation = Some(organisation);

    session.insert("userFull", user_full).await.unwrap();

    Json(GenericResponse {
        success: true,
        message: None,
    })

}

pub async fn get_current_account(session: Session, State(pool): State<MySqlPool>) -> Json<Value> {
    let account_opt: Option<Account> = session.get("account").await.unwrap_or(None);

    if let Some(mut account) = account_opt {
        if let Account::User(ref mut u) = account {
            let row_opt = sqlx::query_as::<_, (String, String, String, Option<i64>)>(
                "SELECT email, first_name, last_name, organisation_id FROM user WHERE id = ?"
            )
            .bind(u.id)
            .fetch_optional(&pool)
            .await;

            if let Ok(Some((email, first_name, last_name, org_id))) = row_opt {
                 u.email = email;
                 u.prenom = first_name;
                 u.nom = last_name;
                 u.id_orga = org_id;

                 let _ = session.insert("account", account.clone()).await;
            }
        }

        Json(json!({
            "success": true,
            "account": account
        }))
    } else {
        Json(json!({
            "success": false,
            "message": "errors.auth.unauthenticated"
        }))
    }
}

pub async fn logout(session: Session) -> Json<Value> {
    session.delete().await.unwrap();
    Json(json!({
        "success": true,
    }))
}