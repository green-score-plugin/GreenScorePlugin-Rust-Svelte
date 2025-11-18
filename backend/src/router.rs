use axum::Router;
use crate::controllers::{home_controller};
use axum::routing::{get};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(home_controller::index))
}
