use axum::Json;
use axum::extract::State;
use serde_json::{json, Value};
use sqlx::{MySqlPool};
use tower_sessions::Session;
use crate::dto::user_full::UserFull;
use crate::models::monitored_website::MonitoredWebsite;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::service::equivalent_service::EquivalentService;
use crate::error::AppError;

pub async fn get_equivalent(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<Value>) -> Result<Json<Value>, AppError> {

    let gco2 = payload.get("gCO2").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let count = payload.get("count").and_then(|v| v.as_u64()).map(|v| v as i32).unwrap_or(3);

    if gco2 <= 0.0 {
        return Err(AppError::BadRequest("Invalid or missing gCO2 parameter".to_string()));
    }

    let user_id: Option<i64> = session.get("user_full").await
        .map_err(|_| AppError::InternalServerError("Session error".to_string()))?
        .and_then(|user_full: UserFull| Some(user_full.user.id));

    let equivalents = EquivalentService::equivalent(&pool, user_id, count, gco2).await
        .map_err(AppError::from)?;

    Ok(Json(json!({
        "success": true,
        "data": equivalents
    })))
}

pub async fn save_monitored_website_data(session: Session, State(pool): State<MySqlPool>, Json(monitored_website): Json<MonitoredWebsite>) -> Result<Json<Value>, AppError> {

    let new_total_footprint = MonitoredWebsiteService::save_monitored_website_data(&pool, &monitored_website).await;

    let mut user_full: UserFull = session.get("user_full").await
        .map_err(|_| AppError::InternalServerError("Session error".to_string()))?
        .ok_or(AppError::BadRequest("User not found in session".to_string()))?; // Keeping BadRequest as per original code, though Unauthorized might be better?

    let current = user_full.user.total_carbon_footprint;
    // logic from original code. save_monitored_website_data returns Result<Option<f64>, Error>.
    // If it fails, new_total_footprint is Err.
    let new_value = match new_total_footprint {
        Ok(Some(val)) => val,
        Ok(None) => current + monitored_website.carbon_footprint,
        Err(e) => return Err(AppError::DatabaseError(e)),
    };

    user_full.user.total_carbon_footprint = new_value;
    let total = user_full.user.total_carbon_footprint;

    session.insert("user_full", user_full).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "total_carbon_footprint": total
    })))
}