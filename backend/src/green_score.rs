// rust
use sqlx::MySqlPool;

pub async fn calculate_green_score(pool: &MySqlPool, carbon_footprint: f64, page: String) -> (String, String) {
    if page == "mo" {

        let avg = match organizations_global_average_carbon_footprint(pool).await {
            Ok(v) if v.is_finite() => v,
            Ok(_) | Err(_) => {
                0.0
            }
        };

        let least = match organizations_least_carbon_footprint(pool).await {
            Ok(v) if v.is_finite() => v,
            Ok(_) | Err(_) => {
                avg
            }
        };

        let max_carbon_footprint = (least - avg) * 2.0;
        let mut scale = (max_carbon_footprint - least) / 7.0;
        if !scale.is_finite() || scale <= 0.0 {
            scale = 0.25;
        }

        let t1 = least + scale;
        let t2 = least + 2.0 * scale;
        let t3 = least + 3.0 * scale;
        let t4 = least + 4.0 * scale;
        let t5 = least + 5.0 * scale;
        let t6 = least + 6.0 * scale;

        let (letter, nomination) = if carbon_footprint < t1 {
            ("A", "Gardien des Écosystèmes")
        } else if carbon_footprint < t2 {
            ("B", "Allié de la Nature")
        } else if carbon_footprint < t3 {
            ("C", "Explorateur Prudent")
        } else if carbon_footprint < t4 {
            ("D", "Voyageur Insouciant")
        } else if carbon_footprint < t5 {
            ("E", "Consommateur Dynamique")
        } else if carbon_footprint < t6 {
            ("F", "Exploitant Intense")
        } else {
            ("G", "Grand consommateur")
        };

        (letter.to_string(), nomination.to_string())

    } else if page == "lpc" {
        let echelle: f64 = 0.25;

        let (letter_green_score, env_nomination) = if carbon_footprint < echelle {
            ("A", "Maître des forêts")
        } else if carbon_footprint < 2.0 * echelle {
            ("B", "Protecteur des Bois")
        } else if carbon_footprint < 3.0 * echelle {
            ("C", "Frère des Arbres")
        } else if carbon_footprint < 4.0 * echelle {
            ("D", "Initié de la Nature")
        } else if carbon_footprint < 5.0 * echelle {
            ("E", "Explorateur Imprudent")
        } else if carbon_footprint < 6.0 * echelle {
            ("F", "Tempête Numérique")
        } else {
            ("G", "Destructeur des Écosystèmes")
        };

        (letter_green_score.to_string(), env_nomination.to_string())
    } else {
        ("N/A".to_string(), "N/A".to_string())
    }
}

async fn organizations_global_average_carbon_footprint(pool: &MySqlPool) -> Result<f64, sqlx::Error> {
    let rows = sqlx::query_as::<_, (f64, i64)>(
        "SELECT AVG(total_carbon_footprint) AS averageConsumption,
        organisation_id AS organisationId
        FROM `user`
        WHERE total_carbon_footprint IS NOT NULL
        AND organisation_id IS NOT NULL
        AND total_carbon_footprint > 0
        GROUP BY organisation_id;",
    )
        .fetch_all(pool)
        .await?;

    if rows.is_empty() {
        return Ok(0.0);
    }

    let sum: f64 = rows.iter().map(|r| r.0).sum();
    let avg: f64 = sum / (rows.len() as f64);

    Ok(avg)
}

async fn organizations_least_carbon_footprint(pool: &MySqlPool) -> Result<f64, sqlx::Error> {
    let row = sqlx::query_as::<_, (f64, i64)>(
        "SELECT SUM(total_carbon_footprint) AS totalConsumption,
        organisation_id AS organisationId
        FROM `user`
        WHERE total_carbon_footprint IS NOT NULL
        AND total_carbon_footprint > 0
        AND organisation_id IS NOT NULL
        GROUP BY organisation_id
        ORDER BY totalConsumption ASC
        LIMIT 1;",
    )
        .fetch_one(pool)
        .await?;

    Ok(row.0)
}
