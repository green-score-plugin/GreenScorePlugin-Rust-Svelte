use axum::Router;
use crate::controllers::{auth_controller, home_controller, account_controller, lpc_controller, mo_controller, my_data_controller, plugin_controller};
use axum::routing::{get, post, patch, delete};
use sqlx::MySqlPool;

pub fn create_router(pool: MySqlPool) -> Router {

    let auth_routes = Router::new()
        .route("/login", post(auth_controller::login))
        .route("/inscription", post(auth_controller::inscription))
        .route("/create_service", post(auth_controller::create_service))
        .route("/get-account", post(auth_controller::get_current_account))
        .route("/logout", post(auth_controller::logout));

    let home_routes = Router::new()
        .route("/advice", get(home_controller::get_advice));

    let account_routes = Router::new()
        .route("/update", patch(account_controller::update_account))
        .route("/delete", delete(account_controller::delete_account))
        // Organization management
        .route("/join-organization", patch(account_controller::join_organization))
        .route("/leave-organization", post(account_controller::leave_organization))
        .route("/my-organization", get(account_controller::get_my_organization))
        .route("/organization/create", post(account_controller::create_organization))
        .route("/organization/members", post(account_controller::get_organisation_member))
        .route("/organization/services", post(account_controller::get_organisation_services))
        .route("/organization/members/remove", post(account_controller::remove_organisation_member))
        .route("/organization/services/delete", post(account_controller::delete_service))
        .route("/organization/services/assign", post(account_controller::assign_user_to_service))
        .route("/organization/services/unassign", post(account_controller::unassign_user_from_service))
        .route("/organization/update", patch(account_controller::update_organisation))
        // Equivalents
        .route("/equivalents", get(account_controller::get_account_all_equivalents))
        .route("/equivalents", patch(account_controller::update_account_equivalents));

    let pages_routes = Router::new()
        .route("/derniere-page-consultee", get(lpc_controller::lpc))
        .route("/mon-organisation", get(mo_controller::mo))
        .route("/mes-donnees", get(my_data_controller::my_data));

    let plugin_routes = Router::new()
        .route("/get-account", post(auth_controller::get_current_account))
        .route("/equivalent", post(plugin_controller::get_equivalent))
        .route("/save_monitored_website_data", post(plugin_controller::save_monitored_website_data));

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/home", home_routes)
        .nest("/account", account_routes)
        .nest("/plugin", plugin_routes)
        .merge(pages_routes)
        .with_state(pool)
}