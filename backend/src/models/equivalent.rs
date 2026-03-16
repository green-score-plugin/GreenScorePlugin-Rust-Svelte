use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Equivalent {
    pub name: String,
    pub value: f64,
    pub icon: String,
}