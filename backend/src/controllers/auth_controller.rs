use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;
use tower_sessions::Session;
use serde::{Deserialize, Serialize};
use rand::Rng;
use crate::models::{User, Organisation, Account};

#[derive(Deserialize)]
pub struct InscriptionRequest {
    pub email: String,
    pub password: String,
    pub lastname: String,
    pub firstname: String,
}

#[derive(Serialize)]
pub struct InscriptionResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub account: Option<Account>,
}

#[derive(Deserialize)]
pub struct InscriptionOrgaRequest {
    pub orga_name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siret : Option<String>,
    pub email: String,
    pub password: String,
}


fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}


fn generate_organisation_code() -> String {
    const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    const LENGTH: usize = 8;

    let mut rng = rand::rng();

    (0..LENGTH)
        .map(|_| {
            let idx = rng.random_range(0..CHARACTERS.len());
            CHARACTERS[idx] as char
        })
        .collect()
}

pub async fn login(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> Json<Value> {

    let row = match sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, email, CAST(password AS CHAR) as password, CAST(roles AS CHAR) as roles  FROM user WHERE email = ?",
    )
        .bind(payload["email"].as_str().unwrap_or(""))
        .fetch_optional(&pool)
        .await {
        Ok(u) => u,
        Err(_) => return Json(json!({
        "success": false,
        "message": "Une erreur est survenue lors de la connexion"
        })),
    };

    if row.is_none() {
        return Json(json!({
            "success": false,
            "message": "Adresse e-mail ou mot de passe invalide",
        }));
    }

    let (user_id, email, password_hash, roles) = row.unwrap();
    let password = payload["password"].as_str().unwrap_or("");
    if !bcrypt::verify(password, &password_hash).unwrap_or(false) {
        Json(json!({
            "success": false,
            "message": "Adresse e-mail ou mot de passe invalide",
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
                Err(e) => {
                    return Json(json!({
                "success": false,
                "message": format!("Erreur récupération utilisateur: {}", e)
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
                "message": "Une erreur est survenue lors de la connexion"
            }));
                }
            }
        }

        Json(json!({
            "success": false,
            "message": "Email ou mot de passe invalide"
        }))
    }
}

pub async fn inscription(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<InscriptionRequest>) -> Json<InscriptionResponse> {
    let email = payload.email.trim();

    let user_exists = sqlx::query("SELECT id FROM user WHERE email = ?")
        .bind(email)
        .fetch_optional(&pool)
        .await;

    match user_exists {
        Ok(Some(_)) => {
            return Json(InscriptionResponse {
                success: false,
                message: Some("Cet email est déjà utilisé".to_string()),
            });
        }
        Err(e) => {
            return Json(InscriptionResponse {
                success: false,
                message: Some(format!("Erreur vérification email: {}", e)),
            });
        }
        Ok(None) => {}
    }

    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => return Json(InscriptionResponse {
            success: false,
            message: Some(format!("Erreur de hashage: {}", e)),
        }),
    };

    match sqlx::query(
        "INSERT INTO user (email, roles, password, first_name, last_name) VALUES (?, ?, ?, ?, ?)"
    )
        .bind(email)
        .bind("[\"ROLE_USER\"]")
        .bind(&password_hash)
        .bind(payload.firstname.trim())
        .bind(payload.lastname.trim())
        .execute(&pool)
        .await {
        Ok(result) => {
            let user_id = result.last_insert_id() as i64;

            let account = Account::User(User {
                id: user_id,
                email: email.to_string(),
                prenom: payload.firstname.trim().to_string(),
                nom: payload.lastname.trim().to_string(),
                id_orga: None,
            });

            session.insert("account", account).await.unwrap();

            Json(InscriptionResponse {
                success: true,
                message: None,
            })
        },
        Err(e) => Json(InscriptionResponse {
            success: false,
            message: Some(format!("Erreur d'inscription: {}", e)),
        }),
    }
}

pub async fn inscription_orga(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<InscriptionOrgaRequest>) -> Json<Value> {
    let email = payload.email.trim();

    let user_exists = sqlx::query("SELECT id FROM user WHERE email = ?")
        .bind(email)
        .fetch_optional(&pool)
        .await;

    match user_exists {
        Ok(Some(_)) => {
            return Json(json!({
                "success": false,
                "message": "Cet email est déjà utilisé"
            }));
        }
        Err(e) => {
             return Json(json!({
                "success": false,
                "message": format!("Erreur vérification email: {}", e)
            }));
        }
        Ok(None) => {}
    }

    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("Erreur de hashage: {}", e)
        })),
    };

    let organisation_code = generate_organisation_code();
    let organisation_id: i64;

    let role : &str = "[\"ROLE_ORGANISATION\"]";

    match sqlx::query(
        "INSERT INTO user (email, roles, password) VALUES (?, ?, ?)"
    )
        .bind(email)
        .bind(&role)
        .bind(&password_hash)
        .execute(&pool)
        .await {
        Ok(result) => {
            let user_id = result.last_insert_id() as i64;
            let orga_name = payload.orga_name.trim();
            let siret = payload.siret.as_ref().map(|s| s.trim().to_string());

            match sqlx::query(
                "INSERT INTO organisation (organisation_name, organisation_code, city, siret, admin_id) VALUES (?, ?, ?, ?, ?)"
            )
                .bind(orga_name)
                .bind(&organisation_code)
                .bind("France")
                .bind(&siret)
                .bind(user_id)
                .execute(&pool)
                .await {
                Ok(_res) => { organisation_id = _res.last_insert_id() as i64;},
                Err(e) => {
                    let _ = sqlx::query("DELETE FROM user WHERE id = ?").bind(user_id).execute(&pool).await;
                    return Json(json!({
                        "success": false,
                        "message": format!("Erreur création organisation: {}", e)
                    }));
                }
            };

            let account = Account::Organisation(Organisation {
                id: organisation_id,
                nom: orga_name.to_string(),
                siret,
                code: organisation_code.clone(),
                admin_id : user_id
            });
            session.insert("account", account.clone()).await.unwrap();


            Json(json!({
                "success": true,
                "account" : account
            }))
        },
        Err(e) => Json(json!({
            "success": false,
            "message": format!("Erreur création utilisateur: {}", e)
        })),
    }
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
            "message": "Non authentifié"
        }))
    }
}

pub async fn logout(session: Session) -> Json<Value> {
    session.delete().await.unwrap();
    Json(json!({
        "success": true,
    }))
}