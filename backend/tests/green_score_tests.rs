use backend::green_score::calculate_green_score;
use sqlx::{MySql, Pool, Row};

// --- Helper to seed data ---
async fn seed_data(pool: &Pool<MySql>) -> (i64, i64) {
    // Create an organisation
    let org_result = sqlx::query("INSERT INTO organisation (organisation_name, organisation_code) VALUES (?, ?)")
        .bind("Test Org")
        .bind("TEST_CODE")
        .execute(pool)
        .await
        .expect("Failed to insert organization");
    let org_id = org_result.last_insert_id() as i64;

    // Create a user in that organisation
    let user_result = sqlx::query(
        "INSERT INTO user (email, roles, password, total_carbon_footprint, organisation_id, est_admin) VALUES (?, '[]', 'pass', ?, ?, 0)"
    )
    .bind("user1@example.com")
    .bind(100.0)
    .bind(org_id)
    .execute(pool)
    .await
    .expect("Failed to insert user 1");
    let u1_id = user_result.last_insert_id() as i64;

    // Create another user
    sqlx::query(
        "INSERT INTO user (email, roles, password, total_carbon_footprint, organisation_id, est_admin) VALUES (?, '[]', 'pass', ?, ?, 0)"
    )
    .bind("user2@example.com")
    .bind(200.0)
    .bind(org_id)
    .execute(pool)
    .await
    .expect("Failed to insert user 2");

    // Organisation stats:
    // Avg = (100+200)/2 = 150.
    // Total (Least because only 1 org) = 300.

    // User stats (Global):
    // Avg = 150.
    // Least = 100.

    (org_id, u1_id)
}

// --- LPC Tests (No DB required, using dummy pool if possible or real one) ---

#[sqlx::test]
async fn test_lpc_grade_a(pool: Pool<MySql>) {
    let (grade, _) = calculate_green_score(&pool, 0.1, "lpc".to_string()).await;
    assert_eq!(grade, "A");
}

#[sqlx::test]
async fn test_lpc_grade_b(pool: Pool<MySql>) {
    let (grade, _) = calculate_green_score(&pool, 0.3, "lpc".to_string()).await;
    assert_eq!(grade, "B");
}

#[sqlx::test]
async fn test_lpc_grade_g(pool: Pool<MySql>) {
    let (grade, _) = calculate_green_score(&pool, 2.0, "lpc".to_string()).await;
    assert_eq!(grade, "G");
}

// --- MO Tests ---

#[sqlx::test]
async fn test_mo_normal(pool: Pool<MySql>) {
    seed_data(&pool).await;

    // Org Avg = 150. Least = 300.
    // carbon_footprint = 150.
    // max = |300 - 150| * 2 = 300.
    // scale = |300 - 300| / 7 = 0. -> default 0.25

    // thresholds: t1 = 300 + 0.25 = 300.25.
    // 150 < 300.25 -> A.

    let (grade, nom) = calculate_green_score(&pool, 150.0, "mo".to_string()).await;
    assert_eq!(grade, "A");
    assert_eq!(nom, "nominations.profile.A");
}

#[sqlx::test]
async fn test_mo_fallback_no_data(pool: Pool<MySql>) {
    // No data in DB. Avg = 0. Least = 0.
    // fallback logic.
    // carbon_footprint = 0.1 -> < 0.25 -> A
    let (grade, _) = calculate_green_score(&pool, 0.1, "mo".to_string()).await;
    assert_eq!(grade, "A");
}

#[sqlx::test]
async fn test_mo_weird_logic_scale(pool: Pool<MySql>) {
    // Try to trigger the scale <= 0.0001 logic explicitly if possible?
    // Already hit in test_mo_normal because scale was 0.
}

// --- My Data Tests ---

#[sqlx::test]
async fn test_my_data_normal(pool: Pool<MySql>) {
    seed_data(&pool).await;

    // Users global: Avg = 150. Least = 100.
    // Input: 120.

    // avg = 150.
    // least = 100.
    // max = |100 - 150| * 2 = 100.
    // scale = |100 - 100| / 7 = 0. -> default 0.25.

    // t1 = 100 + 0.25 = 100.25.
    // 120 is NOT < 100.25.

    // t2 = 100 + 0.5 = 100.5.
    // ...
    // tx = 100 + 2*0.25 ...

    // Since scale is 0.25, the range is tiny around 100.
    // 120 is > 100 + 6*0.25 = 101.5. -> G

    let (grade, nom) = calculate_green_score(&pool, 120.0, "my_data".to_string()).await;
    assert_eq!(grade, "G");
    assert_eq!(nom, "nominations.profile.G");
}

#[sqlx::test]
async fn test_my_data_fallback(pool: Pool<MySql>) {
    // No data
    let (grade, _) = calculate_green_score(&pool, 0.1, "my_data".to_string()).await;
    assert_eq!(grade, "A");
}

// --- Edge Cases ---

#[sqlx::test]
async fn test_na_negative_footprint(pool: Pool<MySql>) {
    let (grade, _) = calculate_green_score(&pool, -5.0, "mo".to_string()).await;
    assert_eq!(grade, "N/A");
}

#[sqlx::test]
async fn test_na_unknown_page(pool: Pool<MySql>) {
    let (grade, _) = calculate_green_score(&pool, 10.0, "unknown".to_string()).await;
    assert_eq!(grade, "N/A");
}
