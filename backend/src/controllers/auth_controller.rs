use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;

pub async fn login(State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> Json<Value> {
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
        Json(json!({
            "success": true,
            "user": {
                "id": user_id,
                "email": email
            }
        }))
    }
}