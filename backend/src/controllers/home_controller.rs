use axum::response::Html;
use tower_sessions::Session;

pub async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to Axum!</h1>")
}
