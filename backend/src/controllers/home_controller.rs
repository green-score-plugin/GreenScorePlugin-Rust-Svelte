use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::MySqlPool;
use crate::models::advice::Advice;
use crate::service::advice_service::AdiceService;

#[derive(Serialize)]
pub struct AdviceResponse {
    success: bool,
    advice: Vec<Advice>
}

pub async fn get_advice(State(pool): State<MySqlPool>) -> Json<AdviceResponse> {
    let advice = match AdiceService::get_all_advice(&pool).await {
        Ok(advice) => advice,
        Err(_) => return Json(AdviceResponse { success: false, advice: vec![] })
    };

    Json(AdviceResponse { success: true, advice })
}
