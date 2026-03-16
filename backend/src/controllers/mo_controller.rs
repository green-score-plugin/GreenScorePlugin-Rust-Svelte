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


pub async fn mo(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Json<MyOrganizationResponse> {

    let organization = match user_full.organisation {
        Some(org) => org,
         None => return Json(MyOrganizationResponse {
            success: false,
            mo_infos: None,
            advices: vec![],
            letter: None,
            env_nomination: None,
            equivalents: None,
            daily_consumption: vec![],
            weekly_consumption: vec![],
            monthly_consumption: vec![],
            top_polluting_sites: vec![],
        })
    };

    let organization_id = organization.id;
    let user_id = user_full.user.id;

    let organization_informations = OrganisationService::organization_informations(&pool, organization_id, user_id).await.ok();

    let advices: Vec<String> = vec![
        AdviceService::get_one_random_advice(&pool, false).await.unwrap_or_default(),
        AdviceService::get_one_random_advice(&pool, true).await.unwrap_or_default(),
    ];

    let (letter, env_nomination, equivalents) = if let Some(ref infos) = organization_informations {
        let (l, n) = calculate_green_score(&pool, infos.average_daily_carbon_footprint, "mo".to_string()).await;

        let eqs = EquivalentService::equivalent(&pool, Some(user_id), 2, infos.total_consumption).await;
        let eqs = match eqs {
            Ok(v) if !v.is_empty() => Some(v),
            _ => None,
        };

        (Some(l), Some(n), eqs)
    } else {
        (None, None, None)
    };

    let daily_consumption = OrganisationService::get_daily_organization_consumption(&pool, organization_id).await.unwrap_or_default();
    let weekly_consumption = OrganisationService::get_weekly_organization_consumption(&pool, organization_id).await.unwrap_or_default();
    let monthly_consumption = OrganisationService::get_monthly_organization_consumption(&pool, organization_id).await.unwrap_or_default();
    let top_polluting_sites = OrganisationService::get_top5_polluting_sites_by_organization(&pool, organization_id).await.unwrap_or_default();

    Json(MyOrganizationResponse {
        success: true,
        mo_infos: organization_informations,
        advices,
        letter,
        env_nomination,
        equivalents,
        daily_consumption,
        weekly_consumption,
        monthly_consumption,
        top_polluting_sites
    })
}