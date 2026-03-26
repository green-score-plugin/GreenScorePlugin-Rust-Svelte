use sqlx::{MySqlPool, Error};
use crate::models::monitored_website::MonitoredWebsite;
use crate::dto::lpc_dto::LastPageConsultedInfos;
use crate::dto::top_polluting_site_dto::TopPollutingSite;
use crate::dto::consumption_data_point_dto::ConsumptionDataPoint;

pub struct MonitoredWebsiteRepository;

impl MonitoredWebsiteRepository {
    pub async fn save_monitored_website_data(
        pool: &MySqlPool,
        monitored_website: &MonitoredWebsite,
    ) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO monitored_website (url_domain, user_id, queries_quantity, data_transferred, resources, loading_time, carbon_footprint, url_full, country)
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

    pub async fn get_last_search_information_by_user(
        pool: &MySqlPool,
        user_id: i64,
    ) -> Result<Option<LastPageConsultedInfos>, Error> {
        let result = sqlx::query_as::<_, LastPageConsultedInfos>(
            r#"
            SELECT url_full, queries_quantity, carbon_footprint, data_transferred, loading_time, country
            FROM monitored_website
            WHERE user_id = ?
            ORDER BY creation_date DESC
            LIMIT 1
            "#,
        )
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_top5_polluting_sites_by_organization(
        pool: &MySqlPool,
        org_id: i64
    ) -> Result<Vec<TopPollutingSite>, Error> {
        let results = sqlx::query_as::<_, TopPollutingSite>(
            "SELECT
            mw.url_domain,
            SUM(mw.carbon_footprint) as total_footprint
            FROM monitored_website mw
            JOIN user u ON u.id = mw.user_id
            WHERE u.organisation_id = ?
            AND mw.url_domain IS NOT NULL
            GROUP BY mw.url_domain
            ORDER BY total_footprint
            DESC LIMIT 5"
        )
            .bind(org_id)
            .fetch_all(pool)
            .await?;
        Ok(results)
    }

    pub async fn get_top5_polluting_sites_by_user(
        pool: &MySqlPool,
        user_id: i64,
    ) -> Result<Vec<TopPollutingSite>, Error> {
        let result = sqlx::query_as::<_, TopPollutingSite>(
            "SELECT
            url_domain,
            SUM(carbon_footprint) as total_footprint
            FROM monitored_website
            WHERE user_id = ?
            AND url_domain IS NOT NULL
            GROUP BY url_domain
            ORDER BY total_footprint DESC
            LIMIT 5"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn average_daily_carbon_footprint_for_organization(
        pool: &MySqlPool,
        org_id: i64
    ) -> f64 {
        sqlx::query_as::<_, (f64,)>(
            "SELECT ROUND(
                COALESCE(
                    SUM(mw.carbon_footprint) / NULLIF(DATEDIFF(CURDATE(), MIN(DATE(mw.creation_date))) + 1, 0)
                , 0)
            , 2) AS average_daily_carbon_footprint
        FROM monitored_website mw
        JOIN user u ON u.id = mw.user_id
        WHERE u.organisation_id = ?"
        )
            .bind(org_id)
            .fetch_one(pool)
            .await
            .map(|(val,)| val)
            .unwrap_or(0.0)
    }

    pub async fn total_organization_consumption(
        pool: &MySqlPool,
        org_id: i64
    ) -> Result<Option<f64>, Error> {
        sqlx::query_as::<_, (Option<f64>,)>(
   "SELECT SUM(mw.carbon_footprint) as total_consumption
        FROM monitored_website mw
        JOIN user u ON u.id = mw.user_id
        WHERE u.organisation_id = ?",
        )
            .bind(org_id)
            .fetch_one(pool)
            .await
            .map(|(val,)| val)
    }

    pub async fn get_daily_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<ConsumptionDataPoint>, Error> {
        let result = sqlx::query_as::<_, ConsumptionDataPoint>(
            "SELECT
            DATE_FORMAT(creation_date, '%d/%m') as label,
            SUM(carbon_footprint) as value
            FROM monitored_website
            WHERE user_id = ?
            AND creation_date >= DATE_SUB(NOW(), INTERVAL 7 DAY)
            GROUP BY DATE(creation_date), label
            ORDER BY DATE(creation_date) ASC"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_weekly_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<(i32, i32, f64)>, Error> {
        let result = sqlx::query_as::<_, (i32, i32, f64)>(
            "SELECT
            CAST(YEAR(creation_date) AS SIGNED) as year,
            CAST(WEEK(creation_date, 1) AS SIGNED) as week,
            SUM(carbon_footprint) as total
            FROM monitored_website
            WHERE user_id = ?
            AND creation_date >= DATE_SUB(NOW(), INTERVAL 4 WEEK)
            GROUP BY year, week
            ORDER BY year, week ASC"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_monthly_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<ConsumptionDataPoint>, Error> {
        let result = sqlx::query_as::<_, ConsumptionDataPoint>(
            "SELECT
            DATE_FORMAT(creation_date, '%m/%Y') as label,
            SUM(carbon_footprint) as value
            FROM monitored_website
            WHERE user_id = ?
            AND creation_date >= DATE_SUB(NOW(), INTERVAL 12 MONTH)
            GROUP BY YEAR(creation_date), MONTH(creation_date), label
            ORDER BY YEAR(creation_date), MONTH(creation_date) ASC"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_my_average_daily_carbon_footprint(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Vec<(String, f64)>, Error>  {
        let result = sqlx::query_as::<_, (String, f64)>(
            "SELECT CAST(DATE(creation_date) AS CHAR) as day, AVG(carbon_footprint) as daily_average
            FROM monitored_website
            WHERE user_id = ?
            GROUP BY day"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_average_daily_carbon_footprint(
        pool: &MySqlPool
    ) -> Result<Vec<(String, f64)>, Error>  {
        let result = sqlx::query_as::<_, (String, f64)>(
            "SELECT
            CAST(DATE(creation_date) AS CHAR) as day,
            AVG(carbon_footprint) as daily_average
            FROM monitored_website
            GROUP BY day"
        )
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_total_consumption_by_user(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<f64, Error>{
        let result = sqlx::query_scalar::<_, f64>(
            "SELECT SUM(carbon_footprint) FROM monitored_website WHERE user_id = ?"
        )
            .bind(user_id)
            .fetch_one(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_daily_organization_consumption(pool: &MySqlPool, orga_id: i64) -> Result<Vec<ConsumptionDataPoint>, Error>
    {
        let result = sqlx::query_as::<_, ConsumptionDataPoint>(
            "SELECT
            DATE_FORMAT(mw.creation_date, '%d/%m') as label,
            SUM(mw.carbon_footprint) as value
            FROM monitored_website mw
            JOIN user u ON u.id = mw.user_id
            WHERE u.organisation_id = ?
            AND mw.creation_date >= DATE_SUB(NOW(), INTERVAL 7 DAY)
            GROUP BY DATE(mw.creation_date), label
            ORDER BY DATE(mw.creation_date) ASC"
        )
            .bind(orga_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn get_weekly_organization_consumption(pool: &MySqlPool, orga_id: i64) -> Result<Vec<ConsumptionDataPoint>, Error>
    {
        let result = sqlx::query_as::<_, ConsumptionDataPoint>(
            "SELECT CONCAT('Semaine ', WEEK(mw.creation_date, 1)) as label,
                        SUM(mw.carbon_footprint) as value
                  FROM monitored_website mw
                  JOIN user u ON u.id = mw.user_id
                  WHERE u.organisation_id = ?
                        AND mw.creation_date >= DATE_SUB(NOW(), INTERVAL 4 WEEK)
                  GROUP BY WEEK(mw.creation_date, 1), label
                  ORDER BY WEEK(mw.creation_date, 1) ASC"
        )
            .bind(orga_id)
            .fetch_all(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_monthly_organization_consumption(pool: &MySqlPool, org_id: i64) -> Result<Vec<ConsumptionDataPoint>, Error>
    {
        let result = sqlx::query_as::<_, ConsumptionDataPoint>(
            "SELECT DATE_FORMAT(mw.creation_date, '%m/%Y') as label,
                        SUM(mw.carbon_footprint) as value
                  FROM monitored_website mw
                  JOIN user u ON u.id = mw.user_id
                  WHERE u.organisation_id = ?
                        AND mw.creation_date >= DATE_SUB(NOW(), INTERVAL 12 MONTH)
                  GROUP BY MONTH(mw.creation_date), YEAR(mw.creation_date), label
                  ORDER BY YEAR(mw.creation_date), MONTH(mw.creation_date) ASC"
        )
            .bind(org_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }
}