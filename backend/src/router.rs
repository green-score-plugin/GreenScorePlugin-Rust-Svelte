use axum::Router;
use crate::controllers::{home_controller, auth_controller, account_controller};
use axum::routing::{post, get, patch, delete};
use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(home_controller::index))
        .route("/login", post(auth_controller::login))
        .route("/inscription", post(auth_controller::inscription))
        .route("/inscription-organisation", post(auth_controller::inscription_orga))
        .route("/logout", post(auth_controller::logout))
        .route("/get-account", post(auth_controller::get_current_account))
        .route("/update_account", patch(account_controller::update_account))
        .route("/delete_account", delete(account_controller::delete_account))
        .route("/join_organization", patch(account_controller::join_organization))
        .with_state(pool)
}
