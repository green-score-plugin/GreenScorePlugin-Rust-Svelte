use serde::{Deserialize, Serialize};
use crate::models::equivalent::Equivalent;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct MyOrganizationInfos {
    pub name: String,
    pub average_daily_carbon_footprint: f64,
    pub equivalent: Option<Equivalent>,
    pub total_consumption: f64,
}