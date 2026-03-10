use crate::models::equivalent::Equivalent;
use crate::repository::equivalent_repository::EquivalentRepository;
pub struct EquivalentService;

impl EquivalentService {
    pub async fn equivalent(
        pool: &sqlx::MySqlPool,
        user_id: Option<i64>,
        n: i32,
        carbon_footprint_in_kg: f64
    ) -> Result<Vec<Equivalent>, sqlx::Error> {
        match user_id {
            Some(id) => EquivalentRepository::get_n_user_equivalent(pool, id, n, carbon_footprint_in_kg).await,
            None => EquivalentRepository::get_n_equivalent(pool, n, carbon_footprint_in_kg).await,
        }
    }
}