use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    #[serde(default)]
    pub id_organisation: Option<i64>,
    #[serde(default)]
    pub id_service: Option<i64>,
    pub email: String,
    pub prenom: String,
    pub nom: String,
    pub est_admin: bool,
    #[serde(default)]
    pub total_carbon_footprint: f64,
}