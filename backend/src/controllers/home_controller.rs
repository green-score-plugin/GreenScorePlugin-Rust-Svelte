use axum::response::Html;
use tower_sessions::Session;

pub async fn index() -> Html<&'static str> {
    Html("<h1>Welcome to Axum!</h1>")
}

// Exemple d'utilisation de session
// pub async fn test_session(session: Session) -> String {
//     // Lire une valeur
//     let count: usize = session.get("count").await.unwrap().unwrap_or(0);
//
//     // Modifier la session
//     session.insert("count", count + 1).await.unwrap();
//
//     format!("Count: {}", count + 1)
// }
