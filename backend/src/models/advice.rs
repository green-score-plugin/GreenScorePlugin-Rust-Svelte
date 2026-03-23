use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, PartialEq)]
pub struct Advice {
    pub advice: String,
    pub title: String,
    pub icon: String,
    pub is_dev: i64,
}