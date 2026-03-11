use axum::extract::{State, Query};
use axum::Json;
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::dto::user_full::UserFull;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::dto::lpc_dto::LastPageConsultedInfos;
use crate::dto::lpc_dto::LastPageConsultedResponse;

pub async fn lpc(
    session: Session,
    State(pool): State<MySqlPool>,
    Query(params): Query<LastPageConsultedInfos>,
) -> Json<LastPageConsultedResponse> {

    let user_id: Option<i64> = session.get("user_full").await.ok().flatten().map(|user_full: UserFull| user_full.user.id);

    let response = MonitoredWebsiteService::lpc(&pool, user_id, Some(params)).await;

    Json(response.unwrap_or(LastPageConsultedResponse {
        success: false,
        letter: None,
        env_nomination: None,
        equivalents: None,
        advices: vec![],
        lpc_infos: None,
    }))


}
