use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MonitoredWebsite {
    pub id: i64,
    pub url_domain: String,
    pub user_id: i64,
    pub queries_quantity: i64,
    pub data_transferred: i64,
    pub resources: i64,
    pub loading_time: f64,
    pub carbon_footprint: f64,
    pub url_full: String,
    pub country: String,
}