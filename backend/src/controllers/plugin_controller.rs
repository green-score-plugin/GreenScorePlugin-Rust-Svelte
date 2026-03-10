use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use serde_json::{json, Value};
use sqlx::{MySqlPool};
use tower_sessions::Session;
use crate::controllers::helpers;
use crate::dto::user_full::UserFull;
use crate::models::monitored_website::MonitoredWebsite;
use crate::service::monitored_website_service::MonitoredWebsiteService;
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

pub async fn save_monitored_website_data(session: Session, State(pool): State<MySqlPool>, Json(monitored_website): Json<MonitoredWebsite>) -> Json<Value> {

    let new_total_footprint = MonitoredWebsiteService::save_monitored_website_data(&pool, &monitored_website).await;

    let mut user_full: Option<UserFull> = session.get("user_full").await.ok().flatten();

    if let Some(ref mut user_full) = user_full {
        let current = user_full.user.total_carbon_footprint;
        let new_value = new_total_footprint
            .ok()
            .flatten()
            .unwrap_or(current + monitored_website.carbon_footprint);
        user_full.user.total_carbon_footprint = new_value;
        let total = user_full.user.total_carbon_footprint;

        session.insert("user_full", user_full).await.ok();

        Json(json!({
            "success": true,
            "total_carbon_footprint": total
        }))
    }
    else {
        Json(json!({
            "success": false,
            "error": "User not found in session"
        }))
    }
}