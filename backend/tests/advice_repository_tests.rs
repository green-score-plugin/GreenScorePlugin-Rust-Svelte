use backend::repository::advice_repository::AdviceRepository;
use sqlx::MySqlPool;

#[sqlx::test]
async fn devrait_retourner_3_advices_get_all_advice(pool: MySqlPool) {
    // GIVEN


    // WHEN
    let advices = AdviceRepository::get_all_advice(&pool).await.expect("Failed to get advices");

    // THEN
    assert_eq!(advices.len(), 3, "On devrait avoir exactement 3 conseils (ceux de la migration)");
}

#[sqlx::test]
async fn devrait_retourner_luminosite_get_one_random_advice_non_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = false;

    // WHEN
    let dev_text = AdviceRepository::get_one_random_advice_text(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice");

    // THEN
    assert_eq!(dev_text, "Réglez la luminosité...");
}

#[sqlx::test]
async fn devrait_retourner_requetes_ou_fichier_get_one_random_advice_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = true;
    let expected_texts = vec![
        "Optimisez vos requêtes SQL...",
        "Minifiez et compressez..."
    ];

    // WHEN
    let dev_text = AdviceRepository::get_one_random_advice_text(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice");

    // THEN
    assert!(
        expected_texts.contains(&dev_text.as_str()),
        "Le conseil retourné pour les devs devrait être soit 'Optimisez vos requêtes SQL...' soit 'Minfiez et compressez...'"
    );
}