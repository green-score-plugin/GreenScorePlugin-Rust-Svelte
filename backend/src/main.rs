mod router;
mod controllers;
mod session_store;
mod cleanup;

use session_store::MySqlStore;
use sqlx::MySqlPool;
use std::env;
use tower_sessions::{SessionManagerLayer, Expiry};
use time::Duration;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    tokio::spawn(cleanup::cleanup_expired_sessions(pool.clone()));

    let store = MySqlStore::new(pool.clone());
    let _session_layer = SessionManagerLayer::new(store)
        .with_secure(false) //pour localhost
        .with_expiry(Expiry::OnInactivity(Duration::seconds(3600)))
        .with_name("greenscoreweb_sessions")
        .with_same_site(tower_sessions::cookie::SameSite::Lax);

    let app = router::create_router()
                    .layer(_session_layer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
