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

    pub async fn get_all_equivalents_with_selection(pool: &MySqlPool, user_id: i64) -> Result<Vec<(i64, String, String, bool)>, sqlx::Error> {
        sqlx::query_as::<_, (i64, String, String, bool)>(
            "SELECT e.id, e.name, e.icon_thumbnail, (u.user_id IS NOT NULL) AS is_selected
              FROM equivalent e LEFT JOIN user_equivalent u ON e.id = u.equivalent_id AND u.user_id = ?"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await
    }

    pub async fn update_user_equivalents(pool: &MySqlPool, user_id: i64, equivalent_ids: Vec<i64>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query("DELETE FROM user_equivalent WHERE user_id = ?")
            .bind(user_id)
            .execute(&mut *tx)
            .await?;

        for equivalent_id in equivalent_ids {
            sqlx::query("INSERT INTO user_equivalent (user_id, equivalent_id) VALUES (?, ?)")
                .bind(user_id)
                .bind(equivalent_id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}