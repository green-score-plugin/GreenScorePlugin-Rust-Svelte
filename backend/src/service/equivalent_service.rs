use crate::dto::equivalent_dto::EquivalentSelection;
use crate::models::equivalent::Equivalent;
use crate::repository::equivalent_repository::EquivalentRepository;
pub struct EquivalentService;

impl EquivalentService {
    pub async fn equivalent(
        pool: &sqlx::MySqlPool,
        user_id: Option<i64>,
        n: i32,
        carbon_footprint: f64
    ) -> Result<Vec<Equivalent>, sqlx::Error> {

        let carbon_footprint_in_kg = carbon_footprint / 1000.0;

        match user_id {
            Some(id) => EquivalentRepository::get_n_user_equivalent(pool, id, n, carbon_footprint_in_kg).await,
            None => EquivalentRepository::get_n_equivalent(pool, n, carbon_footprint_in_kg).await,
        }
    }

    pub async fn get_all_equivalents_with_selection(pool: &sqlx::MySqlPool, user_id: i64) -> Result<Vec<EquivalentSelection>, sqlx::Error> {
        let raw = EquivalentRepository::get_all_equivalents_with_selection(pool, user_id).await?;

        Ok(raw.into_iter().map(|(id, name, icon_thumbnail, is_selected)| EquivalentSelection {
            id,
            name,
            icon_thumbnail,
            is_selected,
        }).collect())
    }

    pub async fn update_user_equivalents(pool: &sqlx::MySqlPool, user_id: i64, equivalent_ids: Vec<i64>) -> Result<(), String> {
        EquivalentRepository::update_user_equivalents(pool, user_id, equivalent_ids)
            .await
            .map_err(|e| format!("Error updating equivalents: {}", e))
    }
}