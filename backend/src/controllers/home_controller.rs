use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;


pub async fn get_advice(State(pool): State<MySqlPool>)->Json<Value> {
    let row = match sqlx::query_as::<_, (String, String, String)>(
        "SELECT advice, title, icon FROM advice",
    )
        .fetch_optional(&pool)
        .await {
        Ok(u) => u,
        Err(e) => return Json(json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        })),
    };
    Json(json!({
            "success": true,
            "advice": row
        }))
}
