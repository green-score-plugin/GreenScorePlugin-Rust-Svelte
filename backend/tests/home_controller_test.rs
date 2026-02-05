#[cfg(test)]
mod tests {
    use axum::extract::State;
    use sqlx::MySqlPool;
    use backend::controllers::home_controller::get_advice;

    #[sqlx::test]
    async fn test_get_advice_returns_all_advice(pool: MySqlPool) {
        let result = get_advice(State(pool)).await;

        assert!(result.0["success"].as_bool().unwrap());
        assert!(result.0["advice"].is_array());
        let advice_list = result.0["advice"].as_array().unwrap();
        assert!(!advice_list.is_empty());
    }

    #[sqlx::test]
    async fn test_get_advice_contains_expected_fields(pool: MySqlPool) {
        let result = get_advice(State(pool)).await;

        let advice_list = result.0["advice"].as_array().unwrap();
        for advice_item in advice_list {
            assert!(advice_item["advice"].is_string());
            assert!(advice_item["title"].is_string());
            assert!(advice_item["icon"].is_string());
            assert!(advice_item["is_dev"].is_number());
        }
    }

    #[sqlx::test]
    async fn test_get_advice_with_empty_table(pool: MySqlPool) {
        sqlx::query("DELETE FROM advice")
            .execute(&pool)
            .await
            .unwrap();

        let result = get_advice(State(pool)).await;

        assert!(result.0["success"].as_bool().unwrap());
        let advice_list = result.0["advice"].as_array().unwrap();
        assert_eq!(advice_list.len(), 0);
    }

    #[sqlx::test]
    async fn test_get_advice_with_database_error(pool: MySqlPool) {
        // Supprimer la table pour forcer une erreur
        sqlx::query("DROP TABLE IF EXISTS advice")
            .execute(&pool)
            .await
            .unwrap();

        let result = get_advice(State(pool)).await;

        assert_eq!(result.0["status"].as_str().unwrap(), "error");
        assert!(result.0["message"].as_str().unwrap().contains("Database error"));
    }
}

