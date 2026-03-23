use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    #[serde(default)]
    #[sqlx(rename = "organisation_id")]
    pub id_organisation: Option<i64>,
    #[serde(default)]
    #[sqlx(rename = "service_id")]
    pub id_service: Option<i64>,
    pub email: String,
    #[sqlx(rename = "first_name")]
    pub prenom: String,
    #[sqlx(rename = "last_name")]
    pub nom: String,
    pub est_admin: bool,
    #[serde(default)]
    pub total_carbon_footprint: f64,
}