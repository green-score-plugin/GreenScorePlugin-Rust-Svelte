use backend::controllers::home_controller;
use axum::extract::State;
use sqlx::{MySqlPool, Row};

#[sqlx::test]
async fn devrait_retourner_succes_avec_liste_vide_quand_aucune_donnee(pool: MySqlPool) {
    // GIVEN
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
async fn devrait_retourner_succes_avec_donnees_migration(pool: MySqlPool) {
    // GIVEN
    // Les données proviennent de la migration (3 éléments insérés : id 1, 2, 21)

    // WHEN
    let response = home_controller::get_advice(State(pool.clone())).await;

    // THEN
    assert!(response.is_ok(), "La réponse devrait être un succès");
    let advice_response = response.unwrap().0;
    assert_eq!(advice_response.success, true, "L'attribut success devrait être à true");
    assert_eq!(advice_response.advice.len(), 3, "La liste d'advices devrait contenir 3 éléments issus de la migration");

    // Vérification sommaire du contenu (basée sur la migration)
    let titles: Vec<String> = advice_response.advice.iter().map(|a| a.title.clone()).collect();
    assert!(titles.contains(&"Optimisez vos requêtes".to_string()));
    assert!(titles.contains(&"Minifiez vos fichiers".to_string()));
    assert!(titles.contains(&"Écran moins lumineux".to_string()));
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
