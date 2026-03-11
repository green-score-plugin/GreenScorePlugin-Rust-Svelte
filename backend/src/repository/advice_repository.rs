use crate::models::advice::Advice;

pub struct AdviceRepository;

impl AdviceRepository {
    pub async fn get_all_advice(pool: &sqlx::MySqlPool) -> Result<Vec<Advice>, sqlx::Error> {
        sqlx::query_as::<_, Advice>(
            "SELECT advice, title, icon, is_dev FROM advice",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_one_random_advice_text(pool: &sqlx::MySqlPool, id_dev: bool) -> Result<String, sqlx::Error> {
        sqlx::query_scalar::<_, String>(
            "SELECT advice FROM advice WHERE id_dev = ? ORDER BY RAND() LIMIT 1",
        )
        .bind(id_dev)
        .fetch_one(pool)
        .await
    }
}