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
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "success": false,
                "error": "Internal server error"
            })))
        }
    }
}

async fn update_user_footprint(pool: &MySqlPool, user_id: i64, carbon_footprint: f64) -> Result<f64, sqlx::Error> {

    let mut transaction = pool.begin().await?;

    sqlx::query(
        r#"
            UPDATE user
            SET total_carbon_footprint = COALESCE(total_carbon_footprint, 0) + ?
            WHERE id = ?
        "#
    )
        .bind(carbon_footprint)
        .bind(user_id)
        .execute(&mut *transaction)
        .await?;

    let new_total: f64 = sqlx::query_scalar(
        "SELECT total_carbon_footprint FROM user WHERE id = ?"
    )
        .bind(user_id)
        .fetch_one(&mut *transaction)
        .await?;

    transaction.commit().await?;

    Ok(new_total)
}

pub async fn save_monitored_website_data(State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> (StatusCode, Json<Value>) {

    let user_id = payload["userId"].as_i64().unwrap_or(0);
    let queries_quantity = payload["totalRequests"].as_i64().unwrap_or(0);
    let loading_time = payload["loadTime"].as_f64().unwrap_or(0.0);
    let data_transferred = payload["totalTransferredSize"].as_i64().unwrap_or(0);
    let resources = payload["totalResourceSize"].as_i64().unwrap_or(0);
    let carbon_footprint = payload["totalEmissions"].as_f64().unwrap_or(0.0);
    let country = payload["country"].as_str().unwrap_or("Unknown");
    let url_full = payload["url"].as_str().unwrap_or("");
    let url_domain = payload["domain"].as_str().unwrap_or("");

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

    match update_user_footprint(&pool, user_id, carbon_footprint).await {
        Ok(new_total_footprint) => {
            (StatusCode::OK, Json(json!({
                "success": true,
                "message": "Data inserted successfully",
                "insertedId": inserted_id,
                "updatedTotalCarbonFootprint": new_total_footprint
            })))
        },
        Err(_) => {
            (StatusCode::OK, Json(json!({
                "success": true,
                "warning": "Website saved but user total not updated",
                "insertedId": inserted_id
            })))
        }
    }
}