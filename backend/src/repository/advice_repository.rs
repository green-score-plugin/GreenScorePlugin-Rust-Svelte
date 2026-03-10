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
}