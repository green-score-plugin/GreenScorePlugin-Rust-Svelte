use backend::green_score;
use sqlx::MySqlPool;

// ============================================
// Tests LPC (Last Page Consulted)
// ============================================

#[tokio::test]
async fn test_calculate_green_score_lpc_grade_b() {
    let (letter, nomination) = green_score::calculate_green_score(None, 0.3, "lpc".to_string()).await;
    assert_eq!(letter, "B");
    assert_eq!(nomination, "nominations.page.B");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_all_grades() {
    let test_cases = vec![
        (0.1, "A", "nominations.page.A"),
        (0.3, "B", "nominations.page.B"),
        (0.6, "C", "nominations.page.C"),
        (0.9, "D", "nominations.page.D"),
        (1.2, "E", "nominations.page.E"),
        (1.4, "F", "nominations.page.F"),
        (1.6, "G", "nominations.page.G"),
    ];

    for (carbon, expected_letter, expected_nomination) in test_cases {
        let (letter, nomination) = green_score::calculate_green_score(None, carbon, "lpc".to_string()).await;
        assert_eq!(letter, expected_letter);
        assert_eq!(nomination, expected_nomination);
    }
}

#[tokio::test]
async fn test_calculate_green_score_lpc_boundary_a() {
    let (letter, _) = green_score::calculate_green_score(None, 0.24, "lpc".to_string()).await;
    assert_eq!(letter, "A");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_boundary_exact() {
    let (letter, _) = green_score::calculate_green_score(None, 0.25, "lpc".to_string()).await;
    assert_eq!(letter, "B");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_zero() {
    let (letter, nomination) = green_score::calculate_green_score(None, 0.0, "lpc".to_string()).await;
    assert_eq!(letter, "A");
    assert_eq!(nomination, "nominations.page.A");
}

#[tokio::test]
async fn test_calculate_green_score_lpc_negative() {
    let (letter, _) = green_score::calculate_green_score(None, -1.0, "lpc".to_string()).await;
    assert_eq!(letter, "A");
}

// ============================================
// Tests MO (My Organization) sans pool
// ============================================

#[tokio::test]
async fn test_calculate_green_score_zero_carbon() {
    let (letter, _nomination) = green_score::calculate_green_score(None, 0.0, "mo".to_string()).await;
    assert_eq!(letter, "N/A");
}

#[tokio::test]
async fn test_calculate_green_score_mo_without_pool() {
    let (letter, nomination) = green_score::calculate_green_score(None, 100.0, "mo".to_string()).await;
    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");
}

// ============================================
// Tests MY_DATA sans pool
// ============================================

#[tokio::test]
async fn test_calculate_green_score_my_data_without_pool() {
    let (letter, nomination) = green_score::calculate_green_score(None, 1.5, "my_data".to_string()).await;
    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");
}

// ============================================
// Tests page invalide
// ============================================

#[tokio::test]
async fn test_calculate_green_score_invalid_page() {
    let (letter, nomination) = green_score::calculate_green_score(None, 1.0, "invalid".to_string()).await;
    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");
}

// ============================================
// Tests MO (My Organization) avec pool
// ============================================

#[sqlx::test]
async fn test_calculate_green_score_mo_with_pool(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1), (200.0, 1), (150.0, 1),
        (50.0, 2), (75.0, 2),
        (300.0, 3)"
    )
    .execute(&pool)
    .await?;

    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 100.0, "mo".to_string()).await;

    assert_ne!(letter, "N/A");
    assert_ne!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_all_grades(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1), (100.0, 1),
        (50.0, 2), (50.0, 2),
        (200.0, 3), (200.0, 3)"
    )
    .execute(&pool)
    .await?;

    // Test grade A (très bas)
    let (letter, _) = green_score::calculate_green_score(Some(&pool), 51.0, "mo".to_string()).await;
    assert_eq!(letter, "A");

    // Test grade G (très haut)
    let (letter, _) = green_score::calculate_green_score(Some(&pool), 500.0, "mo".to_string()).await;
    assert_eq!(letter, "G");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_empty_database(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 100.0, "mo".to_string()).await;

    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_missing_table(pool: MySqlPool) -> sqlx::Result<()> {
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 100.0, "mo".to_string()).await;

    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_with_null_values(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (NULL, 1), (0, 1), (-10.0, 1),
        (100.0, 1), (200.0, 1)"
    )
    .execute(&pool)
    .await?;

    let (letter, _) = green_score::calculate_green_score(Some(&pool), 150.0, "mo".to_string()).await;

    assert_ne!(letter, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_complete_flow(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (50.0, 1), (60.0, 1), (55.0, 1),
        (100.0, 2), (120.0, 2), (110.0, 2),
        (200.0, 3), (220.0, 3), (210.0, 3)"
    )
    .execute(&pool)
    .await?;

    let (letter1, _) = green_score::calculate_green_score(Some(&pool), 52.0, "mo".to_string()).await;
    let (letter2, _) = green_score::calculate_green_score(Some(&pool), 500.0, "mo".to_string()).await;

    assert_ne!(letter1, "N/A");
    assert_ne!(letter2, "N/A");

    let grades = vec!["A", "B", "C", "D", "E", "F", "G"];
    let pos1 = grades.iter().position(|&g| g == letter1).unwrap_or(10);
    let pos2 = grades.iter().position(|&g| g == letter2).unwrap_or(10);
    assert!(pos1 < pos2);

    Ok(())
}

// ============================================
// Tests MY_DATA avec pool
// ============================================

#[sqlx::test]
async fn test_calculate_green_score_my_data_with_pool(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1), (200.0, 2), (150.0, 3), (50.0, 4)"
    )
    .execute(&pool)
    .await?;

    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 80.0, "my_data".to_string()).await;

    assert_ne!(letter, "N/A");
    assert_ne!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_my_data_empty_database(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 100.0, "my_data".to_string()).await;

    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_my_data_missing_table(pool: MySqlPool) -> sqlx::Result<()> {
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 100.0, "my_data".to_string()).await;

    assert_eq!(letter, "N/A");
    assert_eq!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_my_data_with_null_values(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (NULL, 1), (0, 2), (-10.0, 3),
        (100.0, 4), (200.0, 5)"
    )
    .execute(&pool)
    .await?;

    let (letter, _) = green_score::calculate_green_score(Some(&pool), 150.0, "my_data".to_string()).await;

    assert_ne!(letter, "N/A");

    Ok(())
}

// ============================================
// Tests des cas limites et edge cases
// ============================================

#[sqlx::test]
async fn test_calculate_green_score_mo_all_intermediate_grades(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    // Créer une échelle avec :
    // Org 1: total=200 (100+100)
    // Org 2: total=400 (200+200)
    // Org 3: total=600 (300+300)
    // avg global = (200+400+600)/3 = 400
    // least = 200 (org 1)
    // max_carbon = (200-400)*2 = -400
    // scale = (-400-200)/7 = -85.71 -> devient 0.25 (car <= 0)
    // t1 = 200 + 0.25 = 200.25
    // t2 = 200 + 0.50 = 200.50
    // t3 = 200 + 0.75 = 200.75
    // t4 = 200 + 1.00 = 201.00
    // t5 = 200 + 1.25 = 201.25
    // t6 = 200 + 1.50 = 201.50
    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1), (100.0, 1),
        (200.0, 2), (200.0, 2),
        (300.0, 3), (300.0, 3)"
    )
    .execute(&pool)
    .await?;

    // Test grade A (< t1 = 200.25)
    let (letter, _) = green_score::calculate_green_score(Some(&pool), 200.0, "mo".to_string()).await;
    assert_eq!(letter, "A");

    // Test grade B (>= t1, < t2)
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 200.30, "mo".to_string()).await;
    assert_eq!(letter, "B");
    assert_eq!(nomination, "Allié de la Nature");

    // Test grade C (>= t2, < t3)
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 200.60, "mo".to_string()).await;
    assert_eq!(letter, "C");
    assert_eq!(nomination, "Explorateur Prudent");

    // Test grade D (>= t3, < t4)
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 200.90, "mo".to_string()).await;
    assert_eq!(letter, "D");
    assert_eq!(nomination, "Voyageur Insouciant");

    // Test grade E (>= t4, < t5)
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 201.10, "mo".to_string()).await;
    assert_eq!(letter, "E");
    assert_eq!(nomination, "Consommateur Dynamique");

    // Test grade F (>= t5, < t6)
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 201.40, "mo".to_string()).await;
    assert_eq!(letter, "F");
    assert_eq!(nomination, "Exploitant Intense");

    // Test grade G (>= t6)
    let (letter, _) = green_score::calculate_green_score(Some(&pool), 202.0, "mo".to_string()).await;
    assert_eq!(letter, "G");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_my_data_all_intermediate_grades(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    // Créer une échelle avec least=50, avg=100
    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (50.0, 1), (100.0, 2), (150.0, 3)"
    )
    .execute(&pool)
    .await?;

    // Test grade B
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 50.30, "my_data".to_string()).await;
    assert_eq!(letter, "B");
    assert_eq!(nomination, "Allié de la Nature");

    // Test grade C
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 50.60, "my_data".to_string()).await;
    assert_eq!(letter, "C");
    assert_eq!(nomination, "Explorateur Prudent");

    // Test grade D
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 50.90, "my_data".to_string()).await;
    assert_eq!(letter, "D");
    assert_eq!(nomination, "Voyageur Insouciant");

    // Test grade E
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 51.10, "my_data".to_string()).await;
    assert_eq!(letter, "E");
    assert_eq!(nomination, "Consommateur Dynamique");

    // Test grade F
    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 51.40, "my_data".to_string()).await;
    assert_eq!(letter, "F");
    assert_eq!(nomination, "Exploitant Intense");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_single_organization(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1), (200.0, 1), (150.0, 1)"
    )
    .execute(&pool)
    .await?;

    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 175.0, "mo".to_string()).await;

    // Avec une seule organisation, least = avg, donc scale devient 0.25
    assert_ne!(letter, "N/A");
    assert_ne!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_my_data_single_user(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1)"
    )
    .execute(&pool)
    .await?;

    let (letter, nomination) = green_score::calculate_green_score(Some(&pool), 100.0, "my_data".to_string()).await;

    assert_ne!(letter, "N/A");
    assert_ne!(nomination, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_extreme_values(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (0.001, 1), (0.001, 1),
        (10000.0, 2), (10000.0, 2)"
    )
    .execute(&pool)
    .await?;

    let (letter1, _) = green_score::calculate_green_score(Some(&pool), 0.002, "mo".to_string()).await;
    let (letter2, _) = green_score::calculate_green_score(Some(&pool), 50000.0, "mo".to_string()).await;

    assert_ne!(letter1, "N/A");
    assert_ne!(letter2, "N/A");

    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_my_data_vs_lpc_scale(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    // Test que my_data utilise bien l'échelle dynamique avec pool
    sqlx::query(
        "INSERT INTO user (total_carbon_footprint, organisation_id) VALUES
        (100.0, 1), (200.0, 2)"
    )
    .execute(&pool)
    .await?;

    let (letter_my_data, _) = green_score::calculate_green_score(Some(&pool), 150.0, "my_data".to_string()).await;

    // Devrait utiliser l'échelle dynamique, pas lpc
    assert_ne!(letter_my_data, "N/A");

    Ok(())
}

// ============================================
// Tests directs des fonctions auxiliaires
// ============================================

#[sqlx::test]
async fn test_organizations_global_average_carbon_footprint(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS user (id INT PRIMARY KEY AUTO_INCREMENT, total_carbon_footprint DOUBLE, organisation_id INT)")
        .execute(&pool)
        .await?;

    // Org 1: avg = 200
    // Org 2: avg = 400
    // Global avg = (200 + 400) / 2 = 300
    sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (100.0, 1), (300.0, 1), (400.0, 2)")
        .execute(&pool)
        .await?;

    let avg = green_score::organizations_global_average_carbon_footprint(&pool).await?;
    assert_eq!(avg, 300.0);
    Ok(())
}

#[sqlx::test]
async fn test_organizations_global_average_carbon_footprint_empty(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS user (id INT PRIMARY KEY AUTO_INCREMENT, total_carbon_footprint DOUBLE, organisation_id INT)")
        .execute(&pool)
        .await?;

    let avg = green_score::organizations_global_average_carbon_footprint(&pool).await?;
    assert_eq!(avg, 0.0);
    Ok(())
}

#[sqlx::test]
async fn test_organizations_least_carbon_footprint(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS user (id INT PRIMARY KEY AUTO_INCREMENT, total_carbon_footprint DOUBLE, organisation_id INT)")
        .execute(&pool)
        .await?;

    // Org 1: sum = 200
    // Org 2: sum = 50
    // Org 3: sum = 300
    // Least = 50 (Org 2)
    sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (100.0, 1), (100.0, 1), (50.0, 2), (300.0, 3)")
        .execute(&pool)
        .await?;

    let least = green_score::organizations_least_carbon_footprint(&pool).await?;
    assert_eq!(least, 50.0);
    Ok(())
}

#[sqlx::test]
async fn test_users_global_average_carbon_footprint(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS user (id INT PRIMARY KEY AUTO_INCREMENT, total_carbon_footprint DOUBLE, organisation_id INT)")
        .execute(&pool)
        .await?;

    // (100 + 200 + 300) / 3 = 200
    sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (100.0, 1), (200.0, 2), (300.0, 3)")
        .execute(&pool)
        .await?;

    let avg = green_score::users_global_average_carbon_footprint(&pool).await?;
    assert_eq!(avg, 200.0);
    Ok(())
}

#[sqlx::test]
async fn test_users_least_carbon_footprint(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS user (id INT PRIMARY KEY AUTO_INCREMENT, total_carbon_footprint DOUBLE, organisation_id INT)")
        .execute(&pool)
        .await?;

    // Min is 50
    sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (100.0, 1), (50.0, 2), (300.0, 3)")
        .execute(&pool)
        .await?;

    let least = green_score::users_least_carbon_footprint(&pool).await?;
    assert_eq!(least, 50.0);
    Ok(())
}

#[sqlx::test]
async fn test_calculate_green_score_mo_scale_positive(pool: MySqlPool) -> sqlx::Result<()> {
    // We need least > 2 * avg to get positive scale
    // least is MIN(SUM(footprint)) per org
    // avg is AVG(AVG(footprint)) per org

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user (
            id INT PRIMARY KEY AUTO_INCREMENT,
            total_carbon_footprint DOUBLE,
            organisation_id INT
        )"
    )
    .execute(&pool)
    .await?;

    // Create Org 1 with 10 users, each 10.0
    // Org 1 Total = 100.0. Avg = 10.0.
    for _ in 0..10 {
        sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (10.0, 1)")
            .execute(&pool)
            .await?;
    }

    // Checking helper values first to confirm setup
    let avg = green_score::organizations_global_average_carbon_footprint(&pool).await?;
    assert_eq!(avg, 10.0);

    let least = green_score::organizations_least_carbon_footprint(&pool).await?;
    assert_eq!(least, 100.0);

    // least (100) > 2*avg (20).
    // max_carbon = (100 - 10) * 2 = 180.
    // scale = (180 - 100) / 7 = 80 / 7 = 11.428...

    // t1 = least + scale = 100 + 11.42 = 111.42
    // A < 111.42
    let (letter, _) = green_score::calculate_green_score(Some(&pool), 105.0, "mo".to_string()).await;
    assert_eq!(letter, "A");

    // Check G grade in this scale
    // t6 = least + 6*scale = 100 + 68.57 = 168.57
    // 170 > t6 => G
    let (letter_g, _) = green_score::calculate_green_score(Some(&pool), 170.0, "mo".to_string()).await;
    assert_eq!(letter_g, "G");

    Ok(())
}

#[sqlx::test]
async fn test_organizations_helper_ignores_nulls(pool: MySqlPool) -> sqlx::Result<()> {
    sqlx::query("CREATE TABLE IF NOT EXISTS user (id INT PRIMARY KEY AUTO_INCREMENT, total_carbon_footprint DOUBLE, organisation_id INT)")
        .execute(&pool)
        .await?;

    // User with null org
    sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (100.0, NULL)")
        .execute(&pool)
        .await?;

    // Valid user
    sqlx::query("INSERT INTO user (total_carbon_footprint, organisation_id) VALUES (50.0, 1)")
        .execute(&pool)
        .await?;

    // Avg should be 50.0 (ignoring null org)
    let avg = green_score::organizations_global_average_carbon_footprint(&pool).await?;
    assert_eq!(avg, 50.0);

    let least = green_score::organizations_least_carbon_footprint(&pool).await?;
    assert_eq!(least, 50.0);

    Ok(())
}

