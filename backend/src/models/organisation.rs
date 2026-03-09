use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Organisation {
    pub id: i64,
    pub nom: String,
    pub code: String,
    pub siret: Option<String>
}