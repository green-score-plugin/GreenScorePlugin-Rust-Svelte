#[cfg(test)]
mod tests {
    use axum::extract::{Query, State};
    use sqlx::MySqlPool;
    use std::sync::Arc;
    use tower_sessions::session::Id;
    use tower_sessions::{MemoryStore, Session};
    use backend::controllers::lpc_controller::{lpc, LpcParams};
    use backend::models::{Account, User};

    #[sqlx::test]
    async fn test_lpc_minimal(pool: MySqlPool) {
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

    #[sqlx::test]
    async fn test_lpc_from_session_history(pool: MySqlPool) {
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

    #[sqlx::test]
    async fn test_lpc_via_history_with_session(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let mock_account = Account::User(User::new(
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

    #[sqlx::test]
    async fn test_lpc_data_from_sql_with_success(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let user_id = 999;

        sqlx::query("INSERT INTO user (id, email, password, organisation_id, roles) VALUES (?, ?, ?, NULL, '[]')")
            .bind(user_id)
            .bind("test@test.com")
            .bind("hash")
            .execute(&pool).await.unwrap();

        sqlx::query("INSERT INTO monitored_website (user_id, url_full, queries_quantity, carbon_footprint, data_transferred, loading_time, country, creation_date)
                 VALUES (?, 'http://test.com', 5, 0.5, 10.0, 1.2, 'FR', NOW())")
            .bind(user_id)
            .execute(&pool).await.unwrap();

        let mock_account = Account::User(User::new(
            999,
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
            queries_quantity: None
        };

        let result = lpc(session, State(pool), Query(params)).await;

        assert!(result.0.lpc_infos.is_some());
        assert_eq!(result.0.lpc_infos.unwrap().link, "http://test.com");
    }
}