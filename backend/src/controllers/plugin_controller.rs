use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::{json, Value};
use sqlx::{MySqlPool};
use tower_sessions::Session;
use crate::controllers::helpers;
use crate::models::Account;

pub async fn get_equivalent(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {

    let gco2 = payload.get("gCO2").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let count = payload.get("count").and_then(|v| v.as_i64()).unwrap_or(3);

    if gco2 <= 0.0 {
        return (StatusCode::BAD_REQUEST, Json(json!({
            "success": false,
            "error": "Invalid or missing gCO2 parameter",
        })));
    }

    let account_opt: Option<Account> = session.get("account").await.ok().flatten();

    let result = helpers::equivalent(&pool, gco2, count as i32, account_opt.as_ref()).await;

    let data = result.unwrap_or_default();

    (StatusCode::OK, Json(json!({
        "success": true,
        "data": data
    })))
}

pub async fn save_monitored_website_data(State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {

    let user_id = payload.get("userId").and_then(|v| v.as_i64()).unwrap_or(0);
    let queries_quantity = payload.get("totalRequests").and_then(|v| v.as_i64()).unwrap_or(0);
    let loading_time = payload.get("loadTime").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let data_transferred = payload.get("totalTransferredSize").and_then(|v| v.as_i64()).unwrap_or(0);
    let resources = payload.get("totalResourceSize").and_then(|v| v.as_i64()).unwrap_or(0);
    let carbon_footprint = payload.get("totalEmissions").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let country = payload.get("country").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let url_full = payload.get("url").and_then(|v| v.as_str()).unwrap_or("");
    let url_domain = payload.get("domain").and_then(|v| v.as_str()).unwrap_or("");

    let insert_result = sqlx::query(
        r#"
        INSERT INTO monitored_website
        (url_domain, user_id, queries_quantity, data_transferred, resources, loading_time, carbon_footprint, url_full, country)
        VALUES
        (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
        .bind(url_domain)
        .bind(user_id)
        .bind(queries_quantity)
        .bind(data_transferred)
        .bind(resources)
        .bind(loading_time)
        .bind(carbon_footprint)
        .bind(url_full)
        .bind(country)
        .execute(&pool)
        .await;

    let inserted_id = match insert_result {
        Ok(res) => res.last_insert_id(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success": false, "error": "Internal server error"})));
        }
    };

    let update_status = async {
        let _ = sqlx::query("UPDATE user SET total_carbon_footprint = COALESCE(total_carbon_footprint, 0) + ? WHERE id = ?")
            .bind(carbon_footprint)
            .bind(user_id)
            .execute(&pool)
            .await;

        let new_total: f64 = sqlx::query_scalar("SELECT total_carbon_footprint FROM user WHERE id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await.ok()?;

        Some(new_total)
    }.await;

    match update_status {
        Some(new_total_footprint) => {
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Data inserted successfully",
                "insertedId": inserted_id,
                "updatedTotalCarbonFootprint": new_total_footprint
            })))
        },
        None => {
            (StatusCode::OK, Json(json!({
                "success": true,
                "warning": "Website saved but user total not updated",
                "insertedId": inserted_id
            })))
        }
    }
}