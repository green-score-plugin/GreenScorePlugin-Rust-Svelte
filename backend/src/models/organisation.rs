use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Organisation {
    pub id: i64,
    #[sqlx(rename = "organisation_name")]
    pub nom: String,
    #[sqlx(rename = "organisation_code")]
    pub code: String,
    pub siret: Option<String>
}