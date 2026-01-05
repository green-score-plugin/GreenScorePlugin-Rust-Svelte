use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;
use tower_sessions::Session;
use serde::{Deserialize, Serialize};
use rand::Rng;

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

    let mut rng = rand::thread_rng();

    (0..LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARACTERS.len());
            CHARACTERS[idx] as char
        })
        .collect()
}

pub async fn login(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> Json<Value> {
    let row = match sqlx::query_as::<_, (i64, String, String)>(
        "SELECT id, email, password FROM user WHERE email = ?",
    )
        .bind(payload["email"].as_str().unwrap_or(""))
        .fetch_optional(&pool)
        .await {
        Ok(u) => u,
        Err(e) => return Json(json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        })),
    };

    if row.is_none() {
        return Json(json!({
            "success": false,
            "message": "Adresse e-mail ou mot de passe invalide",
        }));
    }

    let (user_id, email, password_hash) = row.unwrap();
    let password = payload["password"].as_str().unwrap_or("");
    if !bcrypt::verify(password, &password_hash).unwrap_or(false) {
        Json(json!({
            "success": false,
            "message": "Adresse e-mail ou mot de passe invalide",
        }))
    } else {
        session.insert("user_id", user_id).await.unwrap();
        session.insert("email", email).await.unwrap();
        Json(json!({
            "success": true
        }))
    }
}

pub async fn inscription(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<InscriptionRequest>) -> Json<InscriptionResponse> {
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
        .bind(&payload.email)
        .bind("[\"ROLE_USER\"]")
        .bind(&password_hash)
        .bind(&payload.firstname)
        .bind(&payload.lastname)
        .execute(&pool)
        .await {
        Ok(result) => {
            let user_id = result.last_insert_id() as i64;
            session.insert("user_id", user_id).await.unwrap();
            session.insert("email", &payload.email).await.unwrap();

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
    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("Erreur de hashage: {}", e)
        })),
    };

    let organisation_code = generate_organisation_code();

    let orga_id = match sqlx::query(
        "INSERT INTO organisation (organisation_name, organisation_code, city, siret) VALUES (?, ?, ?, ?)"
    )
        .bind(&payload.orga_name)
        .bind(&organisation_code)
        .bind("France")
        .bind(payload.siret)
        .execute(&pool)
        .await {
        Ok(result) => result.last_insert_id() as i64,
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("Erreur création organisation: {}", e)
        })),
    };

    match sqlx::query(
        "INSERT INTO user (is_admin_of_id, email, roles, password) VALUES (?, ?, ?, ?)"
    )
        .bind(&orga_id)
        .bind(&payload.email)
        .bind("[\"ROLE_ORGANISATION\"]")
        .bind(&password_hash)
        .execute(&pool)
        .await {
        Ok(result) => {
            let user_id = result.last_insert_id() as i64;
            session.insert("user_id", user_id).await.unwrap();
            session.insert("email", &payload.email).await.unwrap();

            Json(json!({
                "success": true,
                "code": organisation_code
            }))
        },
        Err(e) => Json(json!({
            "success": false,
            "message": format!("Erreur création utilisateur: {}", e)
        })),
    }
}

