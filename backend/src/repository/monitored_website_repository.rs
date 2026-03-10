use crate::models::monitored_website::MonitoredWebsite;
pub struct MonitoredWebsiteRepository;

impl MonitoredWebsiteRepository {
    pub async fn save_monitored_website_data(
        pool: &sqlx::MySqlPool,
        monitored_website: &MonitoredWebsite,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO monitored_websites (url_domain, user_id, queries_quantity, data_transferred, resources, loading_time, carbon_footprint, url_full, country)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&monitored_website.url_domain)
        .bind(monitored_website.user_id)
        .bind(monitored_website.queries_quantity)
        .bind(monitored_website.data_transferred)
        .bind(monitored_website.resources)
        .bind(monitored_website.loading_time)
        .bind(monitored_website.carbon_footprint)
        .bind(&monitored_website.url_full)
        .bind(&monitored_website.country)
        .execute(pool)
        .await?;
        Ok(())
    }
}