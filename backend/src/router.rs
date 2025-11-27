use axum::Router;
use crate::controllers::{home_controller, auth_controller};
use axum::routing::{post, get};
use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/login", post(auth_controller::login))
        .route("/advice", get(home_controller::get_advice))
        .route("/inscription", post(auth_controller::inscription))
        .route("/inscription-organisation", post(auth_controller::inscription_orga))
        .with_state(pool)
}
