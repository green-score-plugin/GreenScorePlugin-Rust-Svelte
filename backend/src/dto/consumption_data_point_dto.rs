use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ConsumptionDataPoint {
    pub label: String,
    pub value: f64,
}