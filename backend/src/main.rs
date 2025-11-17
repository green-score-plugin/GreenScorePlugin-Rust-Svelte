use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn echo(Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/echo", post(echo));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Serveur lancé sur http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}