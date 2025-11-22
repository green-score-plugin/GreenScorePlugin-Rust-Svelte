use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;
use tower_sessions::Session;
use serde::{Deserialize, Serialize};

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
    let password_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
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