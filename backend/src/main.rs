mod router;
mod controllers;
mod session_store;

use session_store::MySqlStore;
use sqlx::MySqlPool;
use std::env;
use tower_sessions::SessionManagerLayer;


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();


    let store = MySqlStore::new(pool.clone());
    let _session_layer = SessionManagerLayer::new(store)
        .with_secure(false); //pour localhost

    let app = router::create_router()
                    .layer(_session_layer);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
