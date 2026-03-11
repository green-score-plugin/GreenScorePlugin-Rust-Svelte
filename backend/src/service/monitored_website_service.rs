use crate::repository::monitored_website_repository::MonitoredWebsiteRepository;
use crate::models::monitored_website::MonitoredWebsite;
use crate::repository::user_repository::UserRepository;
use crate::dto::lpc_dto::LastPageConsultedInfos;
use crate::service::advice_service::AdviceService;
use crate::green_score::calculate_green_score;
use crate::service::equivalent_service::EquivalentService;
use crate::dto::lpc_dto::LastPageConsultedResponse;

pub struct MonitoredWebsiteService;

impl MonitoredWebsiteService {

    async fn get_last_search_information_by_user(
        pool: &sqlx::MySqlPool,
        user_id: i64,
    ) -> Result<Option<LastPageConsultedInfos>, sqlx::Error> {
        MonitoredWebsiteRepository::get_last_search_information_by_user(pool, user_id).await
    }

    pub async fn save_monitored_website_data(
        pool: &sqlx::MySqlPool,
        monitored_website: &MonitoredWebsite,
    ) -> Result<Option<f64>, sqlx::Error> {
        MonitoredWebsiteRepository::save_monitored_website_data(pool, monitored_website).await?;

        UserRepository::update_total_carbon_footprint_by_id(pool, monitored_website.user_id, monitored_website.carbon_footprint).await?;
        let new_total = UserRepository::find_total_carbon_footprint_by_id(pool, monitored_website.user_id).await?;

        Ok(new_total)
    }

    pub async fn lpc(
        pool: &sqlx::MySqlPool,
        user_id: Option<i64>,
        params: Option<LastPageConsultedInfos>,
    ) -> Result<LastPageConsultedResponse, sqlx::Error> {

        let last_search_info = match params {
            Some(infos) => Some(infos),
            None => match user_id {
                Some(id) => Self::get_last_search_information_by_user(pool, id).await?,
                None => None,
            },
        };

        let advices: Vec<String> = vec![
            AdviceService::get_one_random_advice(pool, false).await?,
            AdviceService::get_one_random_advice(pool, true).await?,
        ];

        let (letter, env_nomination, equivalents) = if let Some(ref infos) = last_search_info {
            let (l, n) = calculate_green_score(None, infos.carbon_footprint, "lpc".to_string()).await;

            let eqs = EquivalentService::equivalent(pool, None, 2, infos.carbon_footprint).await;
            let eqs = match eqs {
                Ok(v) if !v.is_empty() => Some(v),
                _ => None,
            };

            (Some(l), Some(n), eqs)
        } else {
            (None, None, None)
        };

        Ok(LastPageConsultedResponse {
            success: true,
            lpc_infos: last_search_info,
            advices,
            letter,
            env_nomination,
            equivalents,
        })
    }
}