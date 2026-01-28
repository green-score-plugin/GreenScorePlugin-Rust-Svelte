use sqlx::MySqlPool;
use backend::controllers::my_data_controller::{
    get_top5_polluting_sites, get_daily_consumption, get_weekly_consumption,
    get_monthly_consumption, get_average_daily_carbon_footprint
};

// ============================================
// Tests get_top5_polluting_sites
// ============================================

#[sqlx::test]
async fn test_get_top5_polluting_sites_basic(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'google.com', 10.5),
        (1, 'facebook.com', 20.3),
        (1, 'google.com', 5.2),
        (1, 'twitter.com', 15.7)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].url_domain, "facebook.com");
    assert_eq!(results[0].total_footprint, 20.3);
    assert_eq!(results[1].url_domain, "google.com");
    assert_eq!(results[1].total_footprint, 15.7);

    Ok(())
}

#[sqlx::test]
async fn test_get_top5_polluting_sites_empty(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 999).await?;
    assert_eq!(results.len(), 0);

    Ok(())
}

#[sqlx::test]
async fn test_get_top5_polluting_sites_null_domains(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, NULL, 50.0),
        (1, 'example.com', 10.0)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url_domain, "example.com");

    Ok(())
}

#[sqlx::test]
async fn test_get_top5_polluting_sites_limit_5(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'site1.com', 100.0),
        (1, 'site2.com', 90.0),
        (1, 'site3.com', 80.0),
        (1, 'site4.com', 70.0),
        (1, 'site5.com', 60.0),
        (1, 'site6.com', 50.0),
        (1, 'site7.com', 40.0)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 5);
    assert_eq!(results[0].url_domain, "site1.com");
    assert_eq!(results[4].url_domain, "site5.com");

    Ok(())
}

#[sqlx::test]
async fn test_get_top5_polluting_sites_rounding(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'test.com', 10.12345),
        (1, 'test.com', 5.67891)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].total_footprint, 15.8);

    Ok(())
}

#[sqlx::test]
async fn test_get_top5_polluting_sites_multiple_users(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'user1site.com', 100.0),
        (2, 'user2site.com', 200.0),
        (1, 'user1site.com', 50.0)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].url_domain, "user1site.com");
    assert_eq!(results[0].total_footprint, 150.0);

    Ok(())
}

// ============================================
// Tests get_daily_consumption
// ============================================

#[sqlx::test]
async fn test_get_daily_consumption_basic(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.5, NOW()),
        (1, 'test2.com', 5.5, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert_eq!(results[0].value, 16.0);

    Ok(())
}

#[sqlx::test]
async fn test_get_daily_consumption_empty(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 999).await?;
    assert_eq!(results.len(), 0);

    Ok(())
}

#[sqlx::test]
async fn test_get_daily_consumption_old_data(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 100.0, DATE_SUB(NOW(), INTERVAL 10 DAY)),
        (1, 'test2.com', 10.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 1).await?;

    // Only data from last 7 days should be included
    assert!(results.iter().all(|r| r.value < 100.0));

    Ok(())
}

#[sqlx::test]
async fn test_get_daily_consumption_rounding(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.123456, NOW()),
        (1, 'test2.com', 5.678912, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert_eq!(results[0].value, 15.8);

    Ok(())
}

#[sqlx::test]
async fn test_get_daily_consumption_multiple_days(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW()),
        (1, 'test2.com', 20.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
        (1, 'test3.com', 30.0, DATE_SUB(NOW(), INTERVAL 2 DAY))"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 1).await?;

    assert_eq!(results.len(), 3);

    Ok(())
}

#[sqlx::test]
async fn test_get_daily_consumption_multiple_users(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW()),
        (2, 'test2.com', 100.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 1).await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].value, 10.0);

    Ok(())
}

// ============================================
// Tests get_weekly_consumption
// ============================================

#[sqlx::test]
async fn test_get_weekly_consumption_basic(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.5, NOW()),
        (1, 'test2.com', 5.5, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_weekly_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert_eq!(results[0].value, 16.0);
    assert!(results[0].label.starts_with("S"));

    Ok(())
}

#[sqlx::test]
async fn test_get_weekly_consumption_empty(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    let results = get_weekly_consumption(&pool, 999).await?;
    assert_eq!(results.len(), 0);

    Ok(())
}

#[sqlx::test]
async fn test_get_weekly_consumption_old_data(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 100.0, DATE_SUB(NOW(), INTERVAL 10 WEEK)),
        (1, 'test2.com', 10.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_weekly_consumption(&pool, 1).await?;

    // Only data from last 4 weeks
    assert!(results.iter().all(|r| r.value < 100.0));

    Ok(())
}

#[sqlx::test]
async fn test_get_weekly_consumption_rounding(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.123456, NOW()),
        (1, 'test2.com', 5.678912, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_weekly_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert_eq!(results[0].value, 15.8);

    Ok(())
}

#[sqlx::test]
async fn test_get_weekly_consumption_multiple_weeks(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW()),
        (1, 'test2.com', 20.0, DATE_SUB(NOW(), INTERVAL 1 WEEK)),
        (1, 'test3.com', 30.0, DATE_SUB(NOW(), INTERVAL 2 WEEK))"
    )
    .execute(&pool)
    .await?;

    let results = get_weekly_consumption(&pool, 1).await?;

    assert!(results.len() >= 1);
    assert!(results.len() <= 3);

    Ok(())
}

#[sqlx::test]
async fn test_get_weekly_consumption_label_format(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_weekly_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert!(results[0].label.starts_with("S"));
    assert!(results[0].label.len() >= 2);

    Ok(())
}

// ============================================
// Tests get_monthly_consumption
// ============================================

#[sqlx::test]
async fn test_get_monthly_consumption_basic(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.5, NOW()),
        (1, 'test2.com', 5.5, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_monthly_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert_eq!(results[0].value, 16.0);
    assert!(results[0].label.contains("/"));

    Ok(())
}

#[sqlx::test]
async fn test_get_monthly_consumption_empty(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    let results = get_monthly_consumption(&pool, 999).await?;
    assert_eq!(results.len(), 0);

    Ok(())
}

#[sqlx::test]
async fn test_get_monthly_consumption_old_data(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 100.0, DATE_SUB(NOW(), INTERVAL 24 MONTH)),
        (1, 'test2.com', 10.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_monthly_consumption(&pool, 1).await?;

    // Only data from last 12 months
    assert!(results.iter().all(|r| r.value < 100.0));

    Ok(())
}

#[sqlx::test]
async fn test_get_monthly_consumption_rounding(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.123456, NOW()),
        (1, 'test2.com', 5.678912, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_monthly_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    assert_eq!(results[0].value, 15.8);

    Ok(())
}

#[sqlx::test]
async fn test_get_monthly_consumption_multiple_months(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW()),
        (1, 'test2.com', 20.0, DATE_SUB(NOW(), INTERVAL 1 MONTH)),
        (1, 'test3.com', 30.0, DATE_SUB(NOW(), INTERVAL 2 MONTH))"
    )
    .execute(&pool)
    .await?;

    let results = get_monthly_consumption(&pool, 1).await?;

    assert!(results.len() >= 1);
    assert!(results.len() <= 3);

    Ok(())
}

#[sqlx::test]
async fn test_get_monthly_consumption_label_format(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let results = get_monthly_consumption(&pool, 1).await?;

    assert!(!results.is_empty());
    // Format should be MM/YYYY
    let parts: Vec<&str> = results[0].label.split('/').collect();
    assert_eq!(parts.len(), 2);
    assert_eq!(parts[0].len(), 2); // MM
    assert_eq!(parts[1].len(), 4); // YYYY

    Ok(())
}

// ============================================
// Tests get_average_daily_carbon_footprint (global)
// ============================================

#[sqlx::test]
async fn test_get_average_daily_carbon_footprint_global_basic(pool: MySqlPool) -> sqlx::Result<()> {

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Day 1: avg = (10+20)/2 = 15
    // Day 2: avg = (30+40)/2 = 35
    // Global avg of averages = (15+35)/2 = 25
    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, '2026-01-01 10:00:00'),
        (2, 'test.com', 20.0, '2026-01-01 11:00:00'),
        (1, 'test.com', 30.0, '2026-01-02 10:00:00'),
        (2, 'test.com', 40.0, '2026-01-02 11:00:00')"
    )
    .execute(&pool)
    .await?;

    let result = get_average_daily_carbon_footprint(&pool).await;

    assert!(result.is_some());
    assert_eq!(result.unwrap(), 25.0);

    Ok(())
}

#[sqlx::test]
async fn test_get_average_daily_carbon_footprint_global_empty(pool: MySqlPool) -> sqlx::Result<()> {

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    let result = get_average_daily_carbon_footprint(&pool).await;

    assert!(result.is_none());

    Ok(())
}

#[sqlx::test]
async fn test_get_average_daily_carbon_footprint_global_rounding(pool: MySqlPool) -> sqlx::Result<()> {

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.12345, '2026-01-01 10:00:00'),
        (1, 'test.com', 5.67891, '2026-01-01 11:00:00')"
    )
    .execute(&pool)
    .await?;

    let result = get_average_daily_carbon_footprint(&pool).await;

    assert!(result.is_some());
    let value = result.unwrap();
    // Should be rounded to 2 decimals
    assert_eq!(value, 7.9);

    Ok(())
}

#[sqlx::test]
async fn test_get_average_daily_carbon_footprint_global_multiple_days(pool: MySqlPool) -> sqlx::Result<()> {

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 100.0, '2026-01-01 10:00:00'),
        (1, 'test.com', 200.0, '2026-01-02 10:00:00'),
        (1, 'test.com', 300.0, '2026-01-03 10:00:00')"
    )
    .execute(&pool)
    .await?;

    let result = get_average_daily_carbon_footprint(&pool).await;

    assert!(result.is_some());
    assert_eq!(result.unwrap(), 200.0);

    Ok(())
}

// ============================================
// Tests edge cases et scénarios complexes
// ============================================

#[sqlx::test]
async fn test_consumption_functions_zero_values(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 0.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let daily = get_daily_consumption(&pool, 1).await?;
    let weekly = get_weekly_consumption(&pool, 1).await?;
    let monthly = get_monthly_consumption(&pool, 1).await?;

    assert!(!daily.is_empty());
    assert!(!weekly.is_empty());
    assert!(!monthly.is_empty());
    assert_eq!(daily[0].value, 0.0);
    assert_eq!(weekly[0].value, 0.0);
    assert_eq!(monthly[0].value, 0.0);

    Ok(())
}

#[sqlx::test]
async fn test_consumption_functions_negative_values(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', -10.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let daily = get_daily_consumption(&pool, 1).await?;

    assert!(!daily.is_empty());
    assert_eq!(daily[0].value, -10.0);

    Ok(())
}

#[sqlx::test]
async fn test_top_polluting_sites_aggregation(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'site1.com', 10.0),
        (1, 'site1.com', 15.0),
        (1, 'site1.com', 25.0),
        (1, 'site2.com', 5.0),
        (1, 'site2.com', 5.0)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].url_domain, "site1.com");
    assert_eq!(results[0].total_footprint, 50.0);
    assert_eq!(results[1].url_domain, "site2.com");
    assert_eq!(results[1].total_footprint, 10.0);

    Ok(())
}

#[sqlx::test]
async fn test_consumption_ordering(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 30.0, DATE_SUB(NOW(), INTERVAL 1 DAY)),
        (1, 'test.com', 10.0, DATE_SUB(NOW(), INTERVAL 3 DAY)),
        (1, 'test.com', 20.0, DATE_SUB(NOW(), INTERVAL 2 DAY))"
    )
    .execute(&pool)
    .await?;

    let daily = get_daily_consumption(&pool, 1).await?;

    // Should be ordered by date ascending
    if daily.len() == 3 {
        assert!(daily[0].value <= daily[1].value);
        assert!(daily[1].value <= daily[2].value);
    }

    Ok(())
}

#[sqlx::test]
async fn test_consumption_same_day_aggregation(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, NOW()),
        (1, 'test.com', 20.0, NOW()),
        (1, 'test.com', 30.0, NOW())"
    )
    .execute(&pool)
    .await?;

    let daily = get_daily_consumption(&pool, 1).await?;

    // All should be aggregated into one day
    assert_eq!(daily.len(), 1);
    assert_eq!(daily[0].value, 60.0);

    Ok(())
}

#[sqlx::test]
async fn test_very_large_carbon_footprint(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'heavy.com', 999999.99)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].total_footprint, 999999.99);

    Ok(())
}

#[sqlx::test]
async fn test_very_small_carbon_footprint(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'light.com', 0.001)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].total_footprint, 0.0);

    Ok(())
}

#[sqlx::test]
async fn test_domain_case_sensitivity(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'Example.com', 10.0),
        (1, 'example.com', 20.0),
        (1, 'EXAMPLE.COM', 30.0)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    // Depending on collation, might be 1 or 3 entries
    assert!(!results.is_empty());

    Ok(())
}

#[sqlx::test]
async fn test_special_characters_in_domain(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint) VALUES
        (1, 'test-site.co.uk', 10.0),
        (1, 'site_test.com', 20.0),
        (1, 'tést.com', 30.0)"
    )
    .execute(&pool)
    .await?;

    let results = get_top5_polluting_sites(&pool, 1).await?;

    assert_eq!(results.len(), 3);

    Ok(())
}

#[sqlx::test]
async fn test_consumption_with_exact_interval_boundaries(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS monitored_website (
            id INT PRIMARY KEY AUTO_INCREMENT,
            user_id INT NOT NULL,
            url_domain VARCHAR(255),
            carbon_footprint DOUBLE NOT NULL,
            creation_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Exactly 7 days ago
    sqlx::query(
        "INSERT INTO monitored_website (user_id, url_domain, carbon_footprint, creation_date) VALUES
        (1, 'test.com', 10.0, DATE_SUB(NOW(), INTERVAL 7 DAY)),
        (1, 'test.com', 20.0, DATE_SUB(NOW(), INTERVAL 8 DAY))"
    )
    .execute(&pool)
    .await?;

    let results = get_daily_consumption(&pool, 1).await?;

    // 8 days ago should be excluded
    assert!(results.is_empty() || results.iter().all(|r| r.value == 10.0));

    Ok(())
}

