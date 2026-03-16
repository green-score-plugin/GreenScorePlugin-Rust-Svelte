use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct TopPollutingSite {
    pub url_domain: String,
    pub total_footprint: f64,
}