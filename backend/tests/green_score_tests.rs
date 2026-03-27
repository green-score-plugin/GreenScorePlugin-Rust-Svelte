use backend::green_score::calculate_green_score;
use sqlx::{MySql, Pool};

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
        "INSERT INTO user (email, password, total_carbon_footprint, organisation_id, est_admin) VALUES (?, 'pass', ?, ?, 0)"
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
        "INSERT INTO user (email, password, total_carbon_footprint, organisation_id, est_admin) VALUES (?, 'pass', ?, ?, 0)"
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

// --- LPC Tests ---

#[sqlx::test]
async fn devrait_retourner_grade_a_pour_lpc(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 0.1;
    let page = "lpc".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "A", "Le grade devrait être A pour une empreinte de 0.1 sur la page LPC");
}

#[sqlx::test]
async fn devrait_retourner_grade_b_pour_lpc(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 0.3;
    let page = "lpc".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "B", "Le grade devrait être B pour une empreinte de 0.3 sur la page LPC");
}

#[sqlx::test]
async fn devrait_retourner_grade_g_pour_lpc(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 2.0;
    let page = "lpc".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "G", "Le grade devrait être G pour une empreinte de 2.0 sur la page LPC");
}

// --- MO Tests ---

#[sqlx::test]
async fn devrait_retourner_grade_a_pour_mo_normal(pool: Pool<MySql>) {
    // GIVEN
    seed_data(&pool).await;
    let carbon_footprint = 100.0;
    let page = "mo".to_string();

    // WHEN
    let (grade, nom) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "A", "Le grade devrait être A par rapport à la moyenne de l'organisation");
    assert_eq!(nom, "nominations.profile.A");
}

#[sqlx::test]
async fn devrait_retourner_grade_a_pour_mo_sans_donnees(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 0.1;
    let page = "mo".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "A", "Le grade devrait être A par défaut s'il n'y a pas de données");
}

#[sqlx::test]
async fn devrait_gerer_logique_echelle_etrange_mo(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 0.1;
    let page = "mo".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "A", "La fallback doit traiter la demande normalement");
}

// --- My Data Tests ---

#[sqlx::test]
async fn devrait_retourner_grade_g_pour_my_data_normal(pool: Pool<MySql>) {
    // GIVEN
    seed_data(&pool).await;
    let carbon_footprint = 200.0;
    let page = "my_data".to_string();

    // WHEN
    let (grade, nom) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "G", "L'utilisateur consomme plus que le threshold calculé et devrait avoir G");
    assert_eq!(nom, "nominations.profile.G");
}

#[sqlx::test]
async fn devrait_retourner_grade_a_pour_my_data_sans_donnees(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 0.1;
    let page = "my_data".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "A", "Le grade devrait être A par défaut s'il n'y a pas de données (fallback)");
}

// --- Edge Cases ---

#[sqlx::test]
async fn devrait_retourner_na_pour_empreinte_negative(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = -5.0;
    let page = "mo".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "N/A", "L'empreinte négative devrait retourner N/A");
}

#[sqlx::test]
async fn devrait_retourner_na_pour_page_inconnue(pool: Pool<MySql>) {
    // GIVEN
    let carbon_footprint = 10.0;
    let page = "unknown".to_string();

    // WHEN
    let (grade, _) = calculate_green_score(&pool, carbon_footprint, page).await;

    // THEN
    assert_eq!(grade, "N/A", "Une page inconnue testée devrait retourner N/A");
}
