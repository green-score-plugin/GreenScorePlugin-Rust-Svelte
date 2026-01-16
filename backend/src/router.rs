use axum::Router;
use crate::controllers::{home_controller, auth_controller, account_controller, plugin_controller, lpc_controller, mo_controller};
use axum::routing::{post, get, patch, delete};

use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {
    Router::new()
        .route("/login", post(auth_controller::login))
        .route("/advice", get(home_controller::get_advice))
        .route("/inscription", post(auth_controller::inscription))
        .route("/inscription-organisation", post(auth_controller::inscription_orga))
        .route("/logout", post(auth_controller::logout))
        .route("/get-account", post(auth_controller::get_current_account))
        .route("/update_account", patch(account_controller::update_account))
        .route("/delete_account", delete(account_controller::delete_account))
        .route("/join_organization", patch(account_controller::join_organization))
        .route("/leave_organization", post(account_controller::leave_organization))
        .route("/get-my-organization", get(account_controller::get_my_organization))
        .route("/derniere-page-consultee", get(lpc_controller::lpc))
        .route("/mon-organisation", get(mo_controller::mo))
        .route("/mes-donnees", get(my_data_controller::my_data))
        .route("/plugin/get-account", post(auth_controller::get_current_account))
        .route("/plugin/equivalent", post(plugin_controller::get_equivalent))
        .route("/plugin/save_monitored_website_data", post(plugin_controller::save_monitored_website_data))
        .with_state(pool)
}