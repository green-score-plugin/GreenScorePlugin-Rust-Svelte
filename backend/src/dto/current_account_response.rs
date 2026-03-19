use serde::Serialize;
use crate::dto::user_full::UserFull;

#[derive(Serialize)]
pub struct CurrentAccountResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_full: Option<UserFull>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}