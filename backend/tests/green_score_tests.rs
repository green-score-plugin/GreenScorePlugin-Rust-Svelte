use backend::green_score::calculate_green_score;
use sqlx::MySqlPool;

// Helper to get a dummy pool that won't actually connect unless used
// Useful for testing paths that don't touch the DB (like "lpc")
async fn get_dummy_pool() -> MySqlPool {
    // connect_lazy won't fail immediately, only on query execution
    // We use a dummy URL. If code tries to use it, it will fail.
    MySqlPool::connect_lazy("mysql://dummy:dummy@localhost/dummy").unwrap()
}

#[tokio::test]
async fn test_calculate_green_score_lpc_grade_a() {
    let pool = get_dummy_pool().await;
    let (grade, nomination) = calculate_green_score(&pool, 0.1, "lpc".to_string()).await;
    assert_eq!(grade, "A");
    assert_eq!(nomination, "nominations.page.A");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_grade_b() {
    let pool = get_dummy_pool().await;
    // echelle = 0.25. 0.3 is between 0.25 and 0.5 -> B
    let (grade, nomination) = calculate_green_score(&pool, 0.3, "lpc".to_string()).await;
    assert_eq!(grade, "B");
    assert_eq!(nomination, "nominations.page.B");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_grade_g() {
    let pool = get_dummy_pool().await;
    // > 1.5 -> G
    let (grade, nomination) = calculate_green_score(&pool, 2.0, "lpc".to_string()).await;
    assert_eq!(grade, "G");
    assert_eq!(nomination, "nominations.page.G");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_boundary() {
    let pool = get_dummy_pool().await;
    // echelle = 0.25.
    // exactly 0.25 -> should be B. (code uses <, so 0.25 is not < 0.25)
    let (grade, nomination) = calculate_green_score(&pool, 0.25, "lpc".to_string()).await;
    assert_eq!(grade, "B");
    assert_eq!(nomination, "nominations.page.B");
}

// Tests for "mo" and "my_data" require a real database connection and seeded data
// We use sqlx::test for this if available, but since setup might be tricky without
// knowing if migrations run automatically, we add them as ignored or try our best.

#[sqlx::test]
#[ignore] // Ignored by default to avoid breaking CI if DB not present
async fn test_calculate_green_score_mo_integration(pool: MySqlPool) -> sqlx::Result<()> {
    // This test requires a running database locally

    // Seed data: Insert some users with carbon footprints
    // We need to ensure `user` table exists.
    // Assuming migrations have run on the test database created by sqlx::test.

    // Insert user 1: organisation_id = 1, footprint = 100.0
    sqlx::query("INSERT INTO user (email, password, total_carbon_footprint, organisation_id) VALUES (?, ?, ?, ?)")
        .bind("test1@example.com")
        .bind("pass")
        .bind(100.0)
        .bind(1)
        .execute(&pool)
        .await?;

    // Insert user 2: organisation_id = 1, footprint = 200.0
    sqlx::query("INSERT INTO user (email, password, total_carbon_footprint, organisation_id) VALUES (?, ?, ?, ?)")
        .bind("test2@example.com")
        .bind("pass")
        .bind(200.0)
        .bind(1)
        .execute(&pool)
        .await?;

    // Avg for org 1 = 150.0. Least for org 1 = 300.0 (Sum of consumption)
    // Wait, query for least is:
    // SELECT SUM(total_carbon_footprint) ... GROUP BY organisation_id ORDER BY totalConsumption ASC LIMIT 1
    // So least is the organisation with the smallest SUM.
    // Here we only have org 1 with sum = 300.0. So least = 300.0.

    // Avg global is:
    // SELECT AVG(total_carbon_footprint) ... GROUP BY organisation_id
    // Then average of those averages.
    // One org (id 1) -> avg consumption = (100+200)/2 = 150.
    // Global avg = 150.

    // So:
    // avg = 150.0
    // least = 300.0

    // calculate_green_score for "mo"
    // carbon_footprint = 100.0

    // max_carbon_footprint = (300 - 150).abs() * 2 = 150 * 2 = 300
    // scale = (300 - 300).abs() / 7 = 0 / 7 = 0.
    // scale <= 0.0001 -> scale = 0.25 (default fallback)

    // t1 = 300 + 0.25 = 300.25

    // footprint = 100.0 < 300.25 -> A

    let (grade, nomination) = calculate_green_score(&pool, 100.0, "mo".to_string()).await;

    assert_eq!(grade, "A");
    assert_eq!(nomination, "nominations.profile.A");

    Ok(())
}

