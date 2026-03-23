use backend::controllers::home_controller;
use axum::extract::State;
use sqlx::{MySqlPool, Row};

// Helper function to ensure tables exist if migrations fail or are not run
async fn ensure_advice_table(pool: &MySqlPool) {
    sqlx::query("
        CREATE TABLE IF NOT EXISTS advice (
            id INT AUTO_INCREMENT PRIMARY KEY,
            advice TEXT NOT NULL,
            title VARCHAR(255) NOT NULL,
            icon VARCHAR(255) NOT NULL,
            is_dev BOOLEAN NOT NULL DEFAULT 0
        )
    ")
    .execute(pool)
    .await
    .ok(); // Ignore if fails (e.g. already exists)
}

#[sqlx::test]
async fn test_get_advice_empty_success(pool: MySqlPool) {
    // Ensure table exists and is empty
    ensure_advice_table(&pool).await;
    sqlx::query("DELETE FROM advice").execute(&pool).await.unwrap();

    let response = home_controller::get_advice(State(pool.clone())).await;

    assert!(response.is_ok());
    let advice_response = response.unwrap().0;
    assert_eq!(advice_response.success, true);
    assert_eq!(advice_response.advice.len(), 0);
}

#[sqlx::test]
async fn test_get_advice_with_data(pool: MySqlPool) {
    ensure_advice_table(&pool).await;
    // Clear table just in case
    sqlx::query("DELETE FROM advice").execute(&pool).await.unwrap();

    // Insert test data
    sqlx::query("INSERT INTO advice (advice, title, icon, is_dev) VALUES (?, ?, ?, ?)")
        .bind("Use less water")
        .bind("Water Saving")
        .bind("water_icon.png")
        .bind(0)
        .execute(&pool)
        .await
        .expect("Failed to insert advice");

    let response = home_controller::get_advice(State(pool.clone())).await;

    assert!(response.is_ok());
    let advice_response = response.unwrap().0;
    assert_eq!(advice_response.success, true);
    assert_eq!(advice_response.advice.len(), 1);
    assert_eq!(advice_response.advice[0].advice, "Use less water");
}

#[sqlx::test]
async fn test_get_advice_db_error(pool: MySqlPool) {
    // Drop the table to force a database error when querying
    sqlx::query("DROP TABLE IF EXISTS advice").execute(&pool).await.unwrap();

    let response = home_controller::get_advice(State(pool.clone())).await;

    assert!(response.is_err());
    // Verify it returns an AppError
    match response {
        Err(_) => (), // expected
        Ok(_) => panic!("Should have failed due to missing table"),
    }
}

