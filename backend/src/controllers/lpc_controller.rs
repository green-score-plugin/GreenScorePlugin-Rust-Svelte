use axum::extract::{State, Query};
use axum::Json;
use sqlx::MySqlPool;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::dto::lpc_dto::LastPageConsultedInfos;
use crate::dto::lpc_dto::LastPageConsultedResponse;
use crate::middleware::auth::AuthenticatedUser;
use crate::error::AppError;

pub async fn lpc(
    State(pool): State<MySqlPool>,
    AuthenticatedUser(user_full): AuthenticatedUser,
    Query(params): Query<LastPageConsultedInfos>,
) -> Result<Json<LastPageConsultedResponse>, AppError> {

    let user_id = Some(user_full.user.id);

    let response = MonitoredWebsiteService::lpc(&pool, user_id, Some(params)).await
        .map_err(AppError::from)?;

    Ok(Json(response))
}
