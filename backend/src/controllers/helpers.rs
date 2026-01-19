use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Equivalent {
    name: String,
    value: f64,
    icon: String,
}

pub async fn advice(pool: &MySqlPool, is_dev: bool) -> String {
    let is_dev_val: i32 = if is_dev { 1 } else { 0 };

    let result = sqlx::query_as::<_, (String,)>(
        "SELECT advice FROM advice WHERE is_dev = ? ORDER BY RAND() LIMIT 1",
    )
        .bind(is_dev_val)
        .fetch_one(pool)
        .await;

    match result {
        Ok((advice, )) => advice,
        Err(_) => {
            if is_dev {
                "Priorisez des outils et workflows durables pour réduire l'empreinte des développeurs.".to_string()
            } else {
                "Adoptez des usages numériques responsables pour diminuer la consommation énergétique.".to_string()
            }
        }
    }
}

pub async fn equivalent(pool: &MySqlPool, carbon_footprint: f64) -> Option<Equivalent> {
    let carbon_footprint_in_kg = carbon_footprint / 1000.0;

    let result = sqlx::query_as::<_, Equivalent>(
        "SELECT name, ROUND(? * equivalent, 2) as value, icon_thumbnail as icon
         FROM equivalent
         WHERE (? * equivalent) >= 1.0
         ORDER BY RAND()
         LIMIT 1",
    )
        .bind(carbon_footprint_in_kg)
        .bind(carbon_footprint_in_kg)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => Some(row),
        Err(_) => {
            None
        }
    }
}