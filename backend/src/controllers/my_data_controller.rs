use axum::extract::State;
use axum::Json;
use serde::{Serialize};
use sqlx::{Error, MySqlPool};
use crate::green_score::calculate_green_score;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::dto::top_polluting_site_dto::TopPollutingSite;
use crate::dto::consumption_data_point_dto::ConsumptionDataPoint;
use crate::models::equivalent::Equivalent;
use crate::service::advice_service::AdviceService;
use crate::service::equivalent_service::EquivalentService;
use crate::middleware::auth::AuthenticatedUser;
use crate::error::AppError;

#[derive(Serialize)]
pub struct MyDataResponse {
    pub success: bool,
    pub my_average_daily_carbon_footprint: Option<f64>,
    pub average_daily_carbon_footprint: Option<f64>,
    pub message_average_footprint: Option<String>,
    pub total_consumption: Option<f64>,
    pub letter_green_score: Option<String>,
    pub env_nomination: Option<String>,
    pub equivalents: Option<Vec<Equivalent>>,
    pub daily_consumption: Vec<ConsumptionDataPoint>,
    pub weekly_consumption: Vec<ConsumptionDataPoint>,
    pub monthly_consumption: Vec<ConsumptionDataPoint>,
    pub top_polluting_sites: Vec<TopPollutingSite>,
    pub advices: Vec<String>,
}

pub async fn get_top5_polluting_sites(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<TopPollutingSite>, Error> {
    MonitoredWebsiteService::get_top5_polluting_sites_by_user(pool, user_id).await
}

pub async fn get_daily_consumption(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<ConsumptionDataPoint>, Error> {
    MonitoredWebsiteService::get_daily_consumption_by_user(pool, user_id).await
}

pub async fn get_weekly_consumption(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<ConsumptionDataPoint>, Error> {
    MonitoredWebsiteService::get_weekly_consumption_by_user(pool, user_id).await
}

pub async fn get_monthly_consumption(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<ConsumptionDataPoint>, Error> {
    MonitoredWebsiteService::get_monthly_consumption_by_user(pool, user_id).await
}


pub async fn get_my_average_daily_carbon_footprint(
    pool: &MySqlPool,
    user_id: i64
) -> Option<f64> {
    MonitoredWebsiteService::get_my_average_daily_carbon_footprint(pool, user_id).await
}

pub async fn get_average_daily_carbon_footprint(
    pool: &MySqlPool,
) -> Option<f64> {
    MonitoredWebsiteService::get_average_daily_carbon_footprint(pool).await
}

pub async fn get_total_consumption(
    pool: &MySqlPool,
    user_id: i64
) -> Option<f64> {
    MonitoredWebsiteService::get_total_consumption_by_user(pool, user_id).await
}

pub async fn my_data(
    State(pool): State<MySqlPool>,
    AuthenticatedUser(user_full): AuthenticatedUser,
)-> Result<Json<MyDataResponse>, AppError> {

    let user_id = user_full.user.id;

    let my_average_daily_carbon_footprint = get_my_average_daily_carbon_footprint(&pool, user_id).await;
    let average_daily_carbon_footprint = get_average_daily_carbon_footprint(&pool).await;

    let message_average_footprint = match (my_average_daily_carbon_footprint, average_daily_carbon_footprint) {
        (Some(user_avg), Some(global_avg)) => {
            if user_avg < global_avg * 0.8 {
                Some("widgets.common.average_daily_footprint.message.low".to_string())
            } else if user_avg <= global_avg * 1.2 {
                Some("widgets.common.average_daily_footprint.message.average".to_string())
            } else {
                Some("widgets.common.average_daily_footprint.message.high".to_string())
            }
        }
        _ => None,
    };

    let user_footprint = my_average_daily_carbon_footprint.unwrap_or(0.0);

    let (letter_green_score, env_nomination) = calculate_green_score(&pool, user_footprint, "my_data".to_string()).await;
    let equivalents: Vec<Equivalent> = EquivalentService::equivalent(&pool, Some(user_id), 2, user_footprint).await.unwrap_or_default();

    let total_consumption = get_total_consumption(&pool, user_id).await;

    let daily_consumption = get_daily_consumption(&pool, user_id).await.map_err(AppError::from)?;
    let weekly_consumption = get_weekly_consumption(&pool, user_id).await.map_err(AppError::from)?;
    let monthly_consumption = get_monthly_consumption(&pool, user_id).await.map_err(AppError::from)?;

let advices: Vec<String> = vec![
    AdviceService::get_one_random_advice(&pool, false).await.unwrap_or_default(),
    AdviceService::get_one_random_advice(&pool, true).await.unwrap_or_default(),
];

    let top_polluting_sites = get_top5_polluting_sites(&pool, user_id).await.map_err(AppError::from)?;

    Ok(Json(MyDataResponse {
        success: true,
        my_average_daily_carbon_footprint,
        average_daily_carbon_footprint,
        message_average_footprint,
        total_consumption,
        letter_green_score: Some(letter_green_score),
        env_nomination: Some(env_nomination),
        equivalents: Some(equivalents),
        daily_consumption,
        weekly_consumption,
        monthly_consumption,
        top_polluting_sites,
        advices,
    }))
}