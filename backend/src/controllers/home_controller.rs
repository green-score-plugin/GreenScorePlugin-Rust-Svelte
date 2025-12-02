use axum::response::Html;
use axum::extract::State;
use axum::Json;
use serde_json::{json, Value};
use sqlx::MySqlPool;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Advice {
    advice: String,
    title: String,
    icon: String,
    is_dev: i64,
}

pub async fn get_advice(State(pool): State<MySqlPool>) -> Json<Value> {
    let rows = match sqlx::query_as::<_, (String, String, String, i64)>(
        "SELECT advice, title, icon, is_dev FROM advice",
    )
        .fetch_all(&pool)
        .await {
        Ok(rows) => rows,
        Err(e) => return Json(json!({
            "status": "error",
            "message": format!("Database error: {}", e)
        })),
    };

    let advice_list: Vec<Advice> = rows
        .into_iter()
        .map(|(advice, title, icon, is_dev)| Advice { advice, title, icon, is_dev })
        .collect();

    Json(json!({
        "success": true,
        "advice": advice_list
    }))
}

