use backend::service::advice_service::AdviceService;
use sqlx::MySqlPool;

#[sqlx::test]
async fn devrait_retourner_3_advices_via_service(pool: MySqlPool) {
    // GIVEN
    // Les données proviennent des migrations par défaut

    // WHEN
    let advices = AdviceService::get_all_advice(&pool).await.expect("Failed to get advices via service");

    // THEN
    assert_eq!(advices.len(), 3, "On devrait avoir exactement 3 conseils (ceux de la migration) en passant par le service");
}

#[sqlx::test]
async fn devrait_retourner_luminosite_via_service_non_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = false;

    // WHEN
    let dev_text = AdviceService::get_one_random_advice(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice via service");

    // THEN
    assert_eq!(dev_text, "Réglez la luminosité...", "Le service devrait retourner le conseil de luminosité pour un non-dev");
}

#[sqlx::test]
async fn devrait_retourner_requetes_ou_fichier_via_service_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = true;
    let expected_texts = vec![
        "Optimisez vos requêtes SQL...",
        "Minifiez et compressez..."
    ];

    // WHEN
    let dev_text = AdviceService::get_one_random_advice(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice via service");

    // THEN
    assert!(
        expected_texts.contains(&dev_text.as_str()),
        "Le conseil retourné via le service pour les devs devrait être soit 'Optimisez vos requêtes SQL...' soit 'Minifiez et compressez...'"
    );
}

