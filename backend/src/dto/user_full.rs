use crate::models::user::User;
use crate::models::organisation::Organisation;
use crate::models::service::Service;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFull {
    pub user: User,
    pub organisation: Option<Vec<Organisation>>,
    pub service: Option<Service>
}