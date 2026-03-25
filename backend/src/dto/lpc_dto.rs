use crate::models::equivalent::Equivalent;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct LastPageConsultedInfos {
    pub url_full: String,
    pub queries_quantity: i32,
    pub carbon_footprint: f64,
    pub data_transferred: f64,
    pub loading_time: f64,
    pub country: String,
}

#[derive(Serialize)]
pub struct LastPageConsultedResponse {
    pub success: bool,
    pub lpc_infos: Option<LastPageConsultedInfos>,
    pub advices: Vec<String>,
    pub letter: Option<String>,
    pub env_nomination: Option<String>,
    pub equivalents: Option<Vec<Equivalent>>,
}