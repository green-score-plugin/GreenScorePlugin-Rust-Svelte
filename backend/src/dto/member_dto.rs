use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MemberDto {
    pub id: i64,
    #[serde(default)]
    #[sqlx(rename = "service_id")]
    pub id_service: Option<i64>,
    pub email: String,
    #[sqlx(rename = "first_name")]
    pub prenom: String,
    #[sqlx(rename = "last_name")]
    pub nom: String,
    #[serde(default)]
    pub total_carbon_footprint: f64,
    #[sqlx(default)]
    pub service_name: Option<String>
}

