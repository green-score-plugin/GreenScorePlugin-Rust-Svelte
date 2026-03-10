use crate::repository::advice_repository::AdviceRepository;
use crate::models::advice::Advice;
pub struct AdiceService;

impl AdiceService {
    pub async fn get_all_advice(pool: &sqlx::MySqlPool) -> Result<Vec<Advice>, sqlx::Error> {
        AdviceRepository::get_all_advice(pool).await
    }
}