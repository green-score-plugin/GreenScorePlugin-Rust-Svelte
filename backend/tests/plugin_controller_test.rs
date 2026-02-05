#[cfg(test)]
mod tests {
    use axum::extract::State;
    use axum::Json;
    use sqlx::MySqlPool;
    use std::sync::Arc;
    use tower_sessions::session::Id;
    use tower_sessions::{MemoryStore, Session};
    use serde_json::json;
    use backend::controllers::plugin_controller;
    use axum::http::StatusCode;

    #[sqlx::test]
    async fn test_get_equivalent_success(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let payload = json!({
            "gCO2": 10.5,
            "count": 3
        });

        let (status, body) = plugin_controller::get_equivalent(
            session,
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::OK);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        assert!(success, "Response should say success: true");

        let data = body.0.get("data").and_then(|v| v.as_array());
        assert!(data.is_some(), "Response should have data array");
    }

    #[sqlx::test]
    async fn test_get_equivalent_bad_request(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let payload = json!({
            "gCO2": -5.0,
            "count": 3
        });

        let (status, body) = plugin_controller::get_equivalent(
            session,
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::BAD_REQUEST);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(true);
        assert!(!success, "Response should say success: false");
    }

    #[sqlx::test]
    async fn test_get_equivalent_none(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let payload = json!({
            "gCO2": 0.1,
            "count": 3
        });

        let (status, body) = plugin_controller::get_equivalent(
            session,
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::OK);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        assert!(success, "Response should say success: true");

        let data = body.0.get("data").and_then(|v| v.as_array());
        assert!(data.is_some(), "Response should have data array");
        assert!(data.unwrap().is_empty(), "Data array should be empty");
    }

    #[sqlx::test]
    async fn test_save_monitored_website_data_success(pool: MySqlPool) {
        let user_id = 123456;

        sqlx::query("INSERT INTO user (id, email, password, roles) VALUES (?, 'test@plugin.com', 'pass', '[]')")
            .bind(user_id)
            .execute(&pool)
            .await
            .expect("Failed to insert user");

        let payload = json!({
            "userId": user_id,
            "totalRequests": 10,
            "loadTime": 0.5,
            "totalTransferredSize": 1024,
            "totalResourceSize": 2048,
            "totalEmissions": 1.5,
            "country": "FR",
            "url": "https://example.com/page",
            "domain": "example.com"
        });

        let (status, body) = plugin_controller::save_monitored_website_data(
            State(pool.clone()),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::OK);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        assert!(success, "Response should say success: true");

        // Verify user total footprint updated
        let row: (Option<f64>,) = sqlx::query_as("SELECT total_carbon_footprint FROM user WHERE id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to fetch user");

        assert_eq!(row.0.unwrap_or(0.0), 1.5);

        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM monitored_website WHERE user_id = ?")
            .bind(user_id)
            .fetch_one(&pool)
            .await
            .expect("Count query failed");

        assert_eq!(count.0, 1);
    }

    #[sqlx::test]
    async fn test_save_monitored_website_data_invalid_user(pool: MySqlPool) {
        let payload = json!({
            "userId": 999999,
            "totalRequests": 10,
            "loadTime": 0.5,
            "totalTransferredSize": 1024,
            "totalResourceSize": 2048,
            "totalEmissions": 1.5,
            "country": "FR",
            "url": "https://example.com/page",
            "domain": "example.com"
        });

        let (status, body) = plugin_controller::save_monitored_website_data(
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(true);
        assert!(!success);
    }

    #[sqlx::test]
    async fn test_save_monitored_website_data_partial_success(pool: MySqlPool) {
        sqlx::query("ALTER TABLE monitored_website DROP FOREIGN KEY FK_monitored_user")
            .execute(&pool)
            .await
            .expect("Failed to drop FK for test simulation");

        let user_id = 999999; // Non-existent user

        let payload = json!({
            "userId": user_id,
            "totalRequests": 10,
            "loadTime": 0.5,
            "totalTransferredSize": 1024,
            "totalResourceSize": 2048,
            "totalEmissions": 1.5,
            "country": "FR",
            "url": "https://example.com/page",
            "domain": "example.com"
        });

        let (status, body) = plugin_controller::save_monitored_website_data(
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::OK);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        assert!(success, "Response should say success: true despite partial failure");

        let warning = body.0.get("warning").and_then(|v| v.as_str());
        assert_eq!(warning, Some("Website saved but user total not updated"));
    }

    #[sqlx::test]
    async fn test_get_equivalent_defaults_and_session_error(pool: MySqlPool) {
        let store = Arc::new(MemoryStore::default());
        let session = Session::new(Some(Id::default()), store, None);

        let payload = json!({});
        let (status, body) = plugin_controller::get_equivalent(
            session.clone(),
            State(pool.clone()),
            Json(payload)
        ).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.0.get("error").and_then(|v| v.as_str()), Some("Invalid or missing gCO2 parameter"));


        session.insert("account", "not_an_account_struct").await.expect("insert failed");

        let payload_valid = json!({ "gCO2": 10.0 }); // Missing count -> default 3
        let (status2, body2) = plugin_controller::get_equivalent(
            session,
            State(pool),
            Json(payload_valid)
        ).await;

        assert_eq!(status2, StatusCode::OK);
        let success = body2.0.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        assert!(success);
    }

    #[sqlx::test]
    async fn test_save_monitored_website_data_defaults_success(pool: MySqlPool) {
        let user_id = 55555;
        sqlx::query("INSERT INTO user (id, email, password, roles) VALUES (?, 'defaults@test.com', 'pass', '[]')")
            .bind(user_id)
            .execute(&pool)
            .await
            .expect("Failed to insert user");

        let payload = json!({
            "userId": user_id
        });

        let (status, body) = plugin_controller::save_monitored_website_data(
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::OK);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
        assert!(success);
    }

    #[sqlx::test]
    async fn test_save_monitored_website_data_missing_userid(pool: MySqlPool) {
        let payload = json!({
            "totalRequests": 5
        });

        let (status, body) = plugin_controller::save_monitored_website_data(
            State(pool),
            Json(payload)
        ).await;

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

        let success = body.0.get("success").and_then(|v| v.as_bool()).unwrap_or(true);
        assert!(!success, "Should fail due to FK violation");
        let error = body.0.get("error").and_then(|v| v.as_str()).unwrap_or("");
        assert_eq!(error, "Internal server error");
    }

}