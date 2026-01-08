use axum::Router;
use crate::controllers::{home_controller, auth_controller, lpc_controller, my_data_controller};
use axum::routing::{post, get};
use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/", get(home_controller::index))
        .route("/login", post(auth_controller::login))
        .route("/inscription", post(auth_controller::inscription))
        .route("/inscription-organisation", post(auth_controller::inscription_orga))
        .route("/logout", post(auth_controller::logout))
        .route("/get-account", post(auth_controller::get_current_account))
        .route("/derniere-page-consultee", get(lpc_controller::lpc))
        // .route("/mes-donnees", get(my_data_controller::my_data))
        .with_state(pool)
}