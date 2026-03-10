use crate::repository::monitored_website_repository::MonitoredWebsiteRepository;
use crate::models::monitored_website::MonitoredWebsite;
use crate::repository::user_repository::UserRepository;

pub struct MonitoredWebsiteService;

impl MonitoredWebsiteService {
    pub async fn save_monitored_website_data(
        pool: &sqlx::MySqlPool,
        monitored_website: &MonitoredWebsite,
    ) -> Result<Option<f64>, sqlx::Error> {
        MonitoredWebsiteRepository::save_monitored_website_data(pool, monitored_website).await?;

        UserRepository::update_total_carbon_footprint_by_id(pool, monitored_website.user_id, monitored_website.carbon_footprint).await?;
        let new_total = UserRepository::find_total_carbon_footprint_by_id(pool, monitored_website.user_id).await?;

        Ok(new_total)
    }
}