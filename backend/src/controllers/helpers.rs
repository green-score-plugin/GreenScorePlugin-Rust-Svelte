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
        Ok((advice, )) => {
                      advice
        },
        Err(_) => {
            if is_dev {
                "data.advice.default_dev".to_string()
            } else {
                "data.advice.default_user".to_string()
            }
        }
    }
}

pub async fn equivalent(pool: &MySqlPool, carbon_footprint: f64, nb_results: i32) -> Option<Vec<Equivalent>> {
    let carbon_footprint_in_kg = carbon_footprint / 1000.0;

    let result = sqlx::query_as::<_, (String, f64, String)>(
        "SELECT name, ROUND(? * equivalent, 2) as value, icon_thumbnail as icon
         FROM equivalent
         WHERE (? * equivalent) >= 1.0
         ORDER BY RAND()
         LIMIT ?",
    )
        .bind(carbon_footprint_in_kg)
        .bind(carbon_footprint_in_kg)
        .bind(nb_results)
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) if !rows.is_empty() => {
             let mapped_rows = rows.into_iter().map(|(name, value, icon)| {
                Equivalent { name, value, icon }
             }).collect();
             Some(mapped_rows)
        },
        _ => None
    }
}
