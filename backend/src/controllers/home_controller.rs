use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::MySqlPool;
use crate::models::advice::Advice;
use crate::service::advice_service::AdviceService;
use crate::error::AppError;

#[derive(Serialize)]
pub struct AdviceResponse {
    success: bool,
    advice: Vec<Advice>
}

pub async fn get_advice(State(pool): State<MySqlPool>) -> Result<Json<AdviceResponse>, AppError> {
    let advice = AdviceService::get_all_advice(&pool).await
        .map_err(AppError::from)?;

    Ok(Json(AdviceResponse { success: true, advice }))
}
