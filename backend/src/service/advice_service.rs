use crate::repository::advice_repository::AdviceRepository;
use crate::models::advice::Advice;
pub struct AdviceService;

impl AdviceService {
    pub async fn get_all_advice(pool: &sqlx::MySqlPool) -> Result<Vec<Advice>, sqlx::Error> {
        AdviceRepository::get_all_advice(pool).await
    }

    pub async fn get_one_random_advice(pool: &sqlx::MySqlPool, id_dev: bool) -> Result<String, sqlx::Error> {
        AdviceRepository::get_one_random_advice_text(pool, id_dev).await
    }
}