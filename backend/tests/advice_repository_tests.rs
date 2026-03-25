use backend::repository::advice_repository::AdviceRepository;
use sqlx::MySqlPool;

#[sqlx::test]
async fn devrait_retourner_3_advices_get_all_advice(pool: MySqlPool) {
    // GIVEN

    // WHEN
    let advices = AdviceRepository::get_all_advice(&pool).await.expect("Failed to get advices");

    // THEN
    assert_eq!(advices.len(), 40, "On devrait avoir exactement 40 conseils (ceux de la migration)");
}

#[sqlx::test]
async fn devrait_retourner_luminosite_get_one_random_advice_non_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = false;

    // WHEN
    let advice_text = AdviceRepository::get_one_random_advice_text(&pool, is_dev)
        .await
        .expect("Failed to get random non-dev advice");

    // THEN
    // The migration data uses translation keys starting with "data.advice."
    assert!(
        advice_text.starts_with("data.advice."),
        "Le conseil non-dev devrait être une clé de traduction commençant par 'data.advice.', reçu: {}", advice_text
    );
}

#[sqlx::test]
async fn devrait_retourner_requetes_ou_fichier_get_one_random_advice_dev(pool: MySqlPool) {
    // GIVEN
    let is_dev: bool = true;

    // WHEN
    let dev_text = AdviceRepository::get_one_random_advice_text(&pool, is_dev)
        .await
        .expect("Failed to get random dev advice");

    // THEN
    // The migration data uses translation keys starting with "data.advice."
    assert!(
        dev_text.starts_with("data.advice."),
        "Le conseil dev devrait être une clé de traduction commençant par 'data.advice.', reçu: {}", dev_text
    );
}