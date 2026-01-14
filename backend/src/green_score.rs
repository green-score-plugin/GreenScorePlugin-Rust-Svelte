use sqlx::MySqlPool;

pub fn calculate_green_score(pool: &MySqlPool, carbon_footprint: f64, page: String) -> (String, String) {
    if page == "mo" {
        let organizations_global_carbon_footprint = organizations_global_carbon_footprint(&pool);
        ("N/A".to_string(), "N/A".to_string())
    }
    else if page == "lpc" {
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
    }

    else {
        ("N/A".to_string(), "N/A".to_string())
    }
}

async fn organizations_global_carbon_footprint(pool: &MySqlPool) -> Result<f64, sqlx::Error> {
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
