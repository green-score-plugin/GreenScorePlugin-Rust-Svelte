use sqlx::{MySqlPool, Error};
use crate::repository::monitored_website_repository::MonitoredWebsiteRepository;
use crate::models::monitored_website::MonitoredWebsite;
use crate::repository::user_repository::UserRepository;
use crate::dto::lpc_dto::LastPageConsultedInfos;
use crate::service::advice_service::AdviceService;
use crate::green_score::calculate_green_score;
use crate::service::equivalent_service::EquivalentService;
use crate::dto::lpc_dto::LastPageConsultedResponse;
use crate::dto::top_polluting_site_dto::TopPollutingSite;
use crate::dto::consumption_data_point_dto::ConsumptionDataPoint;

pub struct MonitoredWebsiteService;

impl MonitoredWebsiteService {

    async fn get_last_search_information_by_user(
        pool: &MySqlPool,
        user_id: i64,
    ) -> Result<Option<LastPageConsultedInfos>, Error> {
        MonitoredWebsiteRepository::get_last_search_information_by_user(pool, user_id).await
    }

    pub async fn save_monitored_website_data(
        pool: &MySqlPool,
        monitored_website: &MonitoredWebsite,
    ) -> Result<Option<f64>, Error> {
        MonitoredWebsiteRepository::save_monitored_website_data(pool, monitored_website).await?;

        UserRepository::update_total_carbon_footprint_by_id(pool, monitored_website.user_id, monitored_website.carbon_footprint).await?;
        let new_total = UserRepository::find_total_carbon_footprint_by_id(pool, monitored_website.user_id).await?;

        Ok(new_total)
    }

    pub async fn lpc(
        pool: &MySqlPool,
        user_id: Option<i64>,
        params: Option<LastPageConsultedInfos>,
    ) -> Result<LastPageConsultedResponse, Error> {

        if let (Some(u_id), Some(infos)) = (user_id, &params) {
            // Simple domain extraction
            let domain = infos.link
                .split("://")
                .nth(1)
                .unwrap_or(&infos.link)
                .split('/')
                .next()
                .unwrap_or(&infos.link)
                .to_string();

            let website = MonitoredWebsite {
                id: 0, // will be ignored on insert
                url_domain: domain,
                user_id: u_id,
                queries_quantity: infos.queries_quantity as i64,
                data_transferred: infos.data_transferred as i64,
                resources: 0, // Default as it's missing in inputs
                loading_time: infos.loading_time,
                carbon_footprint: infos.carbon_footprint,
                url_full: infos.link.clone(),
                country: infos.country.clone(),
            };

            Self::save_monitored_website_data(pool, &website).await?;
        }

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
            let (l, n) = calculate_green_score(&pool, infos.carbon_footprint, "lpc".to_string()).await;

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

    pub async fn get_top5_polluting_sites_by_user(
        pool: &MySqlPool,
        user_id: i64,
    ) -> Result<Vec<TopPollutingSite>, Error> {
        let top_polluting_sites: Vec<TopPollutingSite> = MonitoredWebsiteRepository::get_top5_polluting_sites_by_user(pool, user_id).await?;

        Ok(top_polluting_sites.into_iter()
            .map(|top_polluting_site: TopPollutingSite| TopPollutingSite {
                url_domain: top_polluting_site.url_domain,
                total_footprint: (top_polluting_site.total_footprint * 100.0).round() / 100.0
            })
            .collect())
    }

    pub async fn average_daily_carbon_footprint_for_organization(pool: &MySqlPool, org_id: i64) -> f64 {
        MonitoredWebsiteRepository::average_daily_carbon_footprint_for_organization(pool, org_id).await
    }

    pub async fn get_daily_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<ConsumptionDataPoint>, Error>  {
        let daily_consumtion: Vec<ConsumptionDataPoint> = MonitoredWebsiteRepository::get_daily_consumption_by_user(pool, user_id).await?;

        Ok(daily_consumtion.into_iter()
            .map(|consumption_data_point: ConsumptionDataPoint| ConsumptionDataPoint {
                label: consumption_data_point.label,
                value: (consumption_data_point.value * 100.0).round() / 100.0
            })
            .collect())
    }

    pub async fn total_organization_consumption(pool: &MySqlPool, org_id: i64) -> Result<Option<f64>, Error> {
        MonitoredWebsiteRepository::total_organization_consumption(pool, org_id).await
    }

    pub async fn get_weekly_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<ConsumptionDataPoint>, Error> {
        let weekly_consumption: Vec<(i32, i32, f64)> = MonitoredWebsiteRepository::get_weekly_consumption_by_user(pool, user_id).await?;

        Ok(weekly_consumption.into_iter()
            .map(|(_, week, value)| ConsumptionDataPoint {
                label: format!("S{}", week),
                value: (value * 100.0).round() / 100.0
            })
            .collect())
    }

    pub async fn get_monthly_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<ConsumptionDataPoint>, Error> {
        let monthly_consumption: Vec<ConsumptionDataPoint> = MonitoredWebsiteRepository::get_monthly_consumption_by_user(pool, user_id).await?;

        Ok(monthly_consumption.into_iter()
            .map(|consumption_data_point: ConsumptionDataPoint| ConsumptionDataPoint {
                label: consumption_data_point.label,
                value: (consumption_data_point.value * 100.0).round() / 100.0
            })
            .collect())
    }

    pub async fn get_my_average_daily_carbon_footprint(
        pool: &MySqlPool,
        user_id: i64
    ) -> Option<f64> {
        let result: Vec<(String, f64)> = MonitoredWebsiteRepository::get_my_average_daily_carbon_footprint(pool, user_id)
            .await
            .ok()?;

        if result.is_empty() {
            return None;
        }

        let sum: f64 = result.iter().map(|(_, avg)| avg).sum();
        let average = sum / result.len() as f64;
        Some((average * 100.0).round() / 100.0)
    }

    pub async fn get_average_daily_carbon_footprint(
        pool: &MySqlPool
    ) -> Option<f64> {
        let result: Vec<(String, f64)> = MonitoredWebsiteRepository::get_average_daily_carbon_footprint(pool)
            .await
            .ok()?;

        if result.is_empty() {
            return None;
        }

        let sum: f64 = result.iter().map(|(_, avg)| avg).sum();
        let average = sum / result.len() as f64;
        Some((average * 100.0).round() / 100.0)
    }

    pub async fn get_total_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Option<f64> {
        let result: Result<f64, Error> = MonitoredWebsiteRepository::get_total_consumption_by_user(pool, user_id).await;

        match result {
            Ok(total) => Some((total * 100.0).round() / 100.0),
            Err(_) => None,
        }
    }
}