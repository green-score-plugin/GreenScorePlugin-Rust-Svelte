#[cfg(test)]
mod tests {
    use axum::extract::{Query, State};
    use backend::controllers::lpc_controller::{lpc, LpcParams};
    use sqlx::MySqlPool;
    use std::sync::Arc;
    use tower_sessions::session::Id;
    use tower_sessions::{MemoryStore, Session};
    use tokio::sync::OnceCell;

    static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

    async fn get_test_pool() -> MySqlPool {
        POOL.get_or_init(|| async {
            let _ = dotenvy::dotenv();
            let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL manquante");
            MySqlPool::connect(&db_url).await.expect("Échec connexion DB")
        }).await.clone()
    }

    #[tokio::test]
    async fn test_lpc_minimal() {
        let pool = get_test_pool().await;

        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let params = LpcParams {
            url_full: Some("https://test.com".into()),
            country: Some("FR".into()),
            total_consu: Some(0.1),
            page_size: Some(100.0),
            loading_time: Some(1.0),
            queries_quantity: Some(5),
        };

        let result = lpc(session, State(pool), Query(params)).await;

        assert!(result.0.success);
    }

    #[tokio::test]
    async fn test_lpc_from_session_history() {
        let pool = get_test_pool().await;
        let store = Arc::new(MemoryStore::default());

        let session = Session::new(Some(Id::default()), store, None);

        let params = LpcParams {
            url_full: None,
            country: None,
            total_consu: None,
            page_size: None,
            loading_time: None,
            queries_quantity: None,
        };

        let result = lpc(session, State(pool), Query(params)).await;

        assert!(result.0.success);
    }
}
