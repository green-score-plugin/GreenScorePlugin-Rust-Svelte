#[cfg(test)]
mod tests {
    use axum::extract::{Query, State};
    use backend::controllers::lpc_controller::{lpc, LpcParams};
    use sqlx::MySqlPool;
    use std::sync::Arc;
    use tower_sessions::session::Id;
    use tower_sessions::{MemoryStore, Session};
    use tokio::sync::OnceCell;
    use backend::models::Account;

    static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

    async fn get_test_pool() -> MySqlPool {
        POOL.get_or_init(|| async {
            let _ = dotenvy::dotenv();
            let db_url = std::env::var("DATABASE_URL_TEST").expect("DATABASE_URL manquante");
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

    #[tokio::test]
    async fn test_lpc_via_history_with_session() {
        let pool = get_test_pool().await;
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mock_account = Account::User(backend::models::User::new(
            1,
            "test@email.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            Some(1)
        ));
        session.insert("account", mock_account).await.unwrap();

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

    #[tokio::test]
    async fn test_lpc_data_from_sql_with_success() {
        let pool = get_test_pool().await;
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        sqlx::query("INSERT IGNORE INTO user (id, email, password) VALUES (?, ?, ?)")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO monitored_website (user_id, url_full, queries_quantity, carbon_footprint, data_transferred, loading_time, country, creation_date)
                 VALUES (?, 'http://test.com', 5, 0.5, 10.0, 1.2, 'FR', NOW())")
            .bind(user_id)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(backend::models::User::new(
            user_id,
            "test@email.com".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            Some(1)
        ));
        session.insert("account", mock_account).await.unwrap();

        let params = LpcParams { url_full: None, country: None, total_consu: None, page_size: None, loading_time: None, queries_quantity: None };
        let result = lpc(session, State(pool), Query(params)).await;

        assert!(result.0.lpc_infos.is_some());
        assert_eq!(result.0.lpc_infos.unwrap().link, "http://test.com");
    }
}