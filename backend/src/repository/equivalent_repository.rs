use sqlx::MySqlPool;
use crate::models::equivalent::Equivalent;

pub struct EquivalentRepository;

impl EquivalentRepository {

    pub async fn get_n_user_equivalent(pool: &MySqlPool,
                                  user_id: i64,
                                  n: i32,
                                  carbon_footprint_in_kg: f64
    ) -> Result<Vec<Equivalent>, sqlx::Error> {
        sqlx::query_as::<_, Equivalent>(
            "SELECT DISTINCT e.name, ROUND(? * e.equivalent, 2) as value, e.icon_thumbnail as icon
             FROM equivalent e
             JOIN user_equivalent ue ON e.id = ue.equivalent_id
             WHERE ue.user_id = ?
             AND (? * e.equivalent) >= 1.0
             ORDER BY RAND()
             LIMIT ?",
        )
        .bind(carbon_footprint_in_kg)
        .bind(user_id)
        .bind(carbon_footprint_in_kg)
        .bind(n)
        .fetch_all(pool)
        .await
    }

    pub async fn get_n_equivalent(pool: &MySqlPool,
                                  n: i32,
                                  carbon_footprint_in_kg: f64
    ) -> Result<Vec<Equivalent>, sqlx::Error> {
        sqlx::query_as::<_, Equivalent>(
            "SELECT name, ROUND(? * equivalent, 2) as value, icon_thumbnail as icon
             FROM equivalent
             WHERE (? * equivalent) >= 1.0
             ORDER BY RAND()
             LIMIT ?",
        )
        .bind(carbon_footprint_in_kg)
        .bind(carbon_footprint_in_kg)
        .bind(n)
        .fetch_all(pool)
        .await
    }

}