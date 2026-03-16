use crate::dto::top_polluting_site_dto::TopPollutingSite;
use crate::green_score::calculate_green_score;
use crate::service::advice_service::AdviceService;
use crate::service::equivalent_service::EquivalentService;
use crate::service::organisation_service::OrganisationService;
use crate::models::equivalent::Equivalent;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::MySqlPool;
use crate::dto::consumption_data_point_dto::ConsumptionDataPoint;
use crate::dto::mo_infos_dto::MyOrganizationInfos;
use crate::middleware::auth::AuthenticatedUser;
use crate::error::AppError;

#[derive(Serialize)]
pub struct MyOrganizationResponse {
    pub success: bool,
    pub mo_infos: Option<MyOrganizationInfos>,
    pub advices: Vec<String>,
    pub letter: Option<String>,
    pub env_nomination: Option<String>,
    pub equivalents: Option<Vec<Equivalent>>,
    pub daily_consumption: Vec<ConsumptionDataPoint>,
    pub weekly_consumption: Vec<ConsumptionDataPoint>,
    pub monthly_consumption: Vec<ConsumptionDataPoint>,
    pub top_polluting_sites: Vec<TopPollutingSite>,
}


pub async fn mo(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Result<Json<MyOrganizationResponse>, AppError> {

    let organization = user_full.organisation.ok_or(AppError::NotFound("User is not in an organization".to_string()))?;

    let organization_id = organization.id;
    let user_id = user_full.user.id;

    let organization_informations = OrganisationService::organization_informations(&pool, organization_id, user_id).await
        .map_err(AppError::from)?;

    let advices: Vec<String> = vec![
        AdviceService::get_one_random_advice(&pool, false).await.unwrap_or_default(),
        AdviceService::get_one_random_advice(&pool, true).await.unwrap_or_default(),
    ];

    let (letter, env_nomination) = calculate_green_score(&pool, organization_informations.average_daily_carbon_footprint, "mo".to_string()).await;

    let equivalents = EquivalentService::equivalent(&pool, Some(user_id), 2, organization_informations.total_consumption).await
        .ok();

    let daily_consumption = OrganisationService::get_daily_organization_consumption(&pool, organization_id).await
        .map_err(AppError::from)?;
    let weekly_consumption = OrganisationService::get_weekly_organization_consumption(&pool, organization_id).await
        .map_err(AppError::from)?;
    let monthly_consumption = OrganisationService::get_monthly_organization_consumption(&pool, organization_id).await
        .map_err(AppError::from)?;
    let top_polluting_sites = OrganisationService::get_top5_polluting_sites_by_organization(&pool, organization_id).await
        .map_err(AppError::from)?;

    Ok(Json(MyOrganizationResponse {
        success: true,
        mo_infos: Some(organization_informations),
        advices,
        letter: Some(letter),
        env_nomination: Some(env_nomination),
        equivalents,
        daily_consumption,
        weekly_consumption,
        monthly_consumption,
        top_polluting_sites
    }))
}