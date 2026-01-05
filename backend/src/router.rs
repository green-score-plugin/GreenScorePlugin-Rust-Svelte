use axum::Router;
use crate::controllers::{home_controller, auth_controller, account_controller};
use axum::routing::{post, get, delete};
use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(home_controller::index))
        .route("/login", post(auth_controller::login))
        .route("/inscription", post(auth_controller::inscription))
        .route("/inscription-organisation", post(auth_controller::inscription_orga))
        .route("/account/delete", delete(account_controller::delete_account))
        .with_state(pool)
}
