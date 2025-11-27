use axum::Router;
use crate::controllers::{home_controller, auth_controller};
use axum::routing::{post, get};
use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/login", post(auth_controller::login))
        .route("/advice", get(home_controller::get_advice))
        .with_state(pool)
}
