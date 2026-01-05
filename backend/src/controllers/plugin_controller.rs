use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde::{Serialize};
use serde_json::{json, Value};
use sqlx::{MySqlPool, FromRow};

#[derive(FromRow, Serialize)]
pub struct Equivalent {
    pub id: i32,
    pub equivalent: f64,
    pub name: String,
    pub icon_thumbnail: String
}

pub async fn get_equivalent(State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {

    let gco2 = payload["gCO2"].as_f64().unwrap_or(0.0);
    let count = payload["count"].as_i64().unwrap_or(0);

    if gco2 <= 0.0 {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "success": false,
            "error": "Invalid or missing gCO2 parameter",
        })));
    }

    if count <= 0 || count > 10 {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "success": false,
            "error": "Invalid count parameter",
        })));
    }

    let kgco2 = gco2 / 1000.0;
    let base_kgco2 = 100.0;
    let ratio = kgco2 / base_kgco2;

    let result = sqlx::query_as::<_, Equivalent>(
        r#"
        SELECT id, ROUND(equivalent * ?, 2) as equivalent, name, icon_thumbnail
        FROM equivalent
        WHERE equivalent * ? > 0.1
        AND equivalent * ? < 500
        ORDER BY RAND()
        LIMIT ?
        "#
    )
        .bind(ratio)
        .bind(ratio)
        .bind(ratio)
        .bind(count)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(equivalents) => {
            (StatusCode::OK, Json(json!({
                "success": true,
                "data": equivalents
            })))
        },
        Err(e) => {
            eprintln!("Database error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "success": false,
                "error": "Internal server error"
            })))
        }
    }
}
