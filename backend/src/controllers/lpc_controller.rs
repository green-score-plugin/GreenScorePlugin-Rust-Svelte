use axum::extract::{State, Query};
use axum::Json;
use sqlx::MySqlPool;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::dto::lpc_dto::LastPageConsultedInfos;
use crate::dto::lpc_dto::LastPageConsultedQuery;
use crate::dto::lpc_dto::LastPageConsultedResponse;
use tower_sessions::Session;
use crate::dto::user_full::UserFull;
use crate::error::AppError;

pub async fn lpc(
    State(pool): State<MySqlPool>,
    session: Session,
    Query(params): Query<LastPageConsultedQuery>,
) -> Result<Json<LastPageConsultedResponse>, AppError> {

    let user_id: Option<i64> = session.get("user_full").await
        .ok()
        .and_then(|user_full: Option<UserFull>| user_full.map(|u| u.user.id));
    let infos: Option<LastPageConsultedInfos> = params.into_infos();

    let response = MonitoredWebsiteService::lpc(&pool, user_id, infos).await
        .map_err(AppError::from)?;

    Ok(Json(response))
}
