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
async fn devrait_retourner_succes_avec_liste_vide_quand_aucune_donnee(pool: MySqlPool) {
    // GIVEN
    ensure_advice_table(&pool).await;
    sqlx::query("DELETE FROM advice").execute(&pool).await.unwrap();

    // WHEN
    let response = home_controller::get_advice(State(pool.clone())).await;

    // THEN
    assert!(response.is_ok(), "La réponse devrait être un succès");
    let advice_response = response.unwrap().0;
    assert_eq!(advice_response.success, true, "L'attribut success devrait être à true");
    assert_eq!(advice_response.advice.len(), 0, "La liste d'advices devrait être vide");
}

#[sqlx::test]
async fn devrait_retourner_succes_avec_une_donnee_quand_donnees_presentes(pool: MySqlPool) {
    // GIVEN
    ensure_advice_table(&pool).await;
    sqlx::query("DELETE FROM advice").execute(&pool).await.unwrap();

    sqlx::query("INSERT INTO advice (advice, title, icon, is_dev) VALUES (?, ?, ?, ?)")
        .bind("Use less water")
        .bind("Water Saving")
        .bind("water_icon.png")
        .bind(0)
        .execute(&pool)
        .await
        .expect("Failed to insert advice");

    // WHEN
    let response = home_controller::get_advice(State(pool.clone())).await;

    // THEN
    assert!(response.is_ok(), "La réponse devrait être un succès");
    let advice_response = response.unwrap().0;
    assert_eq!(advice_response.success, true, "L'attribut success devrait être à true");
    assert_eq!(advice_response.advice.len(), 1, "La liste d'advices devrait contenir exactement 1 élément");
    assert_eq!(advice_response.advice[0].advice, "Use less water", "Le texte de l'advice devrait correspondre à ce qui a été inséré");
}

#[sqlx::test]
async fn devrait_retourner_erreur_quand_table_inexistante(pool: MySqlPool) {
    // GIVEN
    sqlx::query("DROP TABLE IF EXISTS advice").execute(&pool).await.unwrap();

    // WHEN
    let response = home_controller::get_advice(State(pool.clone())).await;

    // THEN
    assert!(response.is_err(), "On s'attend à ce que la requête échoue si la table n'existe pas");
}
