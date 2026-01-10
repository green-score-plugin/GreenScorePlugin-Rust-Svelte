mod router;
mod controllers;
mod session_store;
mod cleanup;
pub mod models;

use session_store::MySqlStore;
use sqlx::MySqlPool;
use std::env;
use axum::http::{header, HeaderValue, Method};
use tower_sessions::{SessionManagerLayer, Expiry};
use time::Duration;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let backend_url = env::var("BACKEND_URL").unwrap_or("localhost:3000".to_string());
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");

    let pool = MySqlPool::connect(&database_url).await.unwrap();

    tokio::spawn(cleanup::cleanup_expired_sessions(pool.clone()));

    let store = MySqlStore::new(pool.clone());
    let session_layer = SessionManagerLayer::new(store)
        .with_secure(false)
        .with_http_only(true)
        .with_same_site(tower_sessions::cookie::SameSite::Lax) // ✅ Change en Lax
        .with_name("greenscoreweb_sessions")
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600)));

    let cors = CorsLayer::new()
        .allow_origin([
            "https://testgreenscore.alwaysdata.net".parse::<HeaderValue>().unwrap(),
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:5173".parse::<HeaderValue>().unwrap()
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::OPTIONS
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::COOKIE,
        ])
        .allow_credentials(true); // obligatoire pour cookies cross-site

    let app = router::create_router(pool)
        .layer(session_layer)
        .layer(cors);


    let listener = tokio::net::TcpListener::bind(backend_url.clone())
        .await
        .unwrap();

    println!("Le serveur a bien restart");
    println!("Server running at {}", backend_url);

    axum::serve(listener, app).await.unwrap();
}
