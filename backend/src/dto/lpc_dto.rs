use crate::models::equivalent::Equivalent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow, PartialEq)]
pub struct LastPageConsultedInfos {
    pub url_full: String,
    pub queries_quantity: i32,
    pub carbon_footprint: f64,
    pub data_transferred: f64,
    pub loading_time: f64,
    pub country: String,
}

#[derive(Debug, Deserialize, Clone, Default, PartialEq)]
pub struct LastPageConsultedQuery {
    pub url_full: Option<String>,
    pub queries_quantity: Option<i32>,
    pub carbon_footprint: Option<f64>,
    pub data_transferred: Option<f64>,
    pub loading_time: Option<f64>,
    pub country: Option<String>,
}

impl LastPageConsultedQuery {
    pub fn into_infos(self) -> Option<LastPageConsultedInfos> {
        Some(LastPageConsultedInfos {
            url_full: self.url_full?,
            queries_quantity: self.queries_quantity?,
            carbon_footprint: self.carbon_footprint?,
            data_transferred: self.data_transferred?,
            loading_time: self.loading_time?,
            country: self.country?,
        })
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct LastPageConsultedResponse {
    pub success: bool,
    pub lpc_infos: Option<LastPageConsultedInfos>,
    pub advices: Vec<String>,
    pub letter: Option<String>,
    pub env_nomination: Option<String>,
    pub equivalents: Option<Vec<Equivalent>>,
}