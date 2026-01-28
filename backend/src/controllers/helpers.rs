use serde::Serialize;
use sqlx::MySqlPool;
use crate::models::Account;

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

pub async fn equivalent(pool: &MySqlPool, carbon_footprint: f64, nb_results: i32, account_opt: Option<&Account>) -> Option<Vec<Equivalent>> {
    let carbon_footprint_in_kg = carbon_footprint / 1000.0;

    let mut use_selection = false;

    if let Some(account) = account_opt {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM user_equivalent WHERE user_id = ?")
            .bind(account.id())
            .fetch_one(pool)
            .await
            .unwrap_or(0);

        if count > 0 {
            use_selection = true;
        }
    }

    let result = if use_selection {
         sqlx::query_as::<_, Equivalent>(
            "SELECT e.name, ROUND(? * e.equivalent, 2) as value, e.icon_thumbnail as icon
             FROM equivalent e
             JOIN user_equivalent ue ON e.id = ue.equivalent_id
             WHERE ue.user_id = ?
             AND (? * e.equivalent) >= 1.0
             ORDER BY RAND()
             LIMIT ?",
        )
            .bind(carbon_footprint_in_kg)
            .bind(account_opt.unwrap().id())
            .bind(carbon_footprint_in_kg)
            .bind(nb_results)
            .fetch_all(pool)
            .await
    } else {
        sqlx::query_as::<_, Equivalent>(
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
            .await
    };

    match result {
        Ok(rows) if !rows.is_empty() => Some(rows),
        Ok(_) | Err(_) => {
            None
        }
    }
}
