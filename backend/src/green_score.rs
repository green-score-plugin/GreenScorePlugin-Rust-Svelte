use sqlx::MySqlPool;

pub async fn calculate_green_score(pool: &MySqlPool, carbon_footprint: f64, page: String) -> (String, String) {
    if page == "mo" || page == "my_data" {
        let mut least: f64 = 0.0;
        let mut avg: f64 = 0.0;

        if page == "mo" {
            avg = match organizations_global_average_carbon_footprint(pool).await {
                Ok(v) if v.is_finite() => v,
                Ok(_) | Err(_) => {
                    0.0
                }
            };

            least = match organizations_least_carbon_footprint(pool).await {
                Ok(v) if v.is_finite() => v,
                Ok(_) | Err(_) => {
                    avg
                }
            };
        }

        if page == "my_data" {
            avg = match users_global_average_carbon_footprint(pool).await {
                Ok(Some(v)) if v.is_finite() => v,
                _ => 0.0
            };

            least = match users_least_carbon_footprint(pool).await {
                Ok(Some(v)) if v.is_finite() => v,
                _ => avg
            };
        }

        if carbon_footprint > 0.0 {
            // If avg is 0 (no other data), we can't compare. But we should give a grade.
            // Let's assume a default scale if avg is missing or invalid.
            // Or if avg is valid but least is invalid.

            let (letter, nomination) = if avg > 0.0 {
                // Normal case with data
                let max_carbon_footprint = (least - avg).abs() * 2.0; // Use abs just in case
                let mut scale = (max_carbon_footprint - least).abs() / 7.0; // Use abs

                // If scale is weird (e.g. least = avg), default to check
                if !scale.is_finite() || scale <= 0.0001 {
                    scale = 0.25;
                }

                let t1 = least + scale;
                let t2 = least + 2.0 * scale;
                let t3 = least + 3.0 * scale;
                let t4 = least + 4.0 * scale;
                let t5 = least + 5.0 * scale;
                let t6 = least + 6.0 * scale;

                if carbon_footprint < t1 { ("A", "nominations.profile.A") }
                else if carbon_footprint < t2 { ("B", "nominations.profile.B") }
                else if carbon_footprint < t3 { ("C", "nominations.profile.C") }
                else if carbon_footprint < t4 { ("D", "nominations.profile.D") }
                else if carbon_footprint < t5 { ("E", "nominations.profile.E") }
                else if carbon_footprint < t6 { ("F", "nominations.profile.F") }
                else { ("G", "nominations.profile.G") }
            } else {
                 // No global data to compare.
                 // Fallback to absolute scale? Or give A?
                 // Let's use the static scale defined for LPC as fallback for consistency
                 let echelle: f64 = 0.25;
                  if carbon_footprint < echelle { ("A", "nominations.profile.A") }
                else if carbon_footprint < 2.0 * echelle { ("B", "nominations.profile.B") }
                else if carbon_footprint < 3.0 * echelle { ("C", "nominations.profile.C") }
                else if carbon_footprint < 4.0 * echelle { ("D", "nominations.profile.D") }
                else if carbon_footprint < 5.0 * echelle { ("E", "nominations.profile.E") }
                else if carbon_footprint < 6.0 * echelle { ("F", "nominations.profile.F") }
                else { ("G", "nominations.profile.G") }
            };

            (letter.to_string(), nomination.to_string())

        } else {
            ("N/A".to_string(), "N/A".to_string())
        }

    } else if page == "lpc" || page == "my_data" { // This condition is now redundant for my_data but ok
        let echelle: f64 = 0.25;

        let (letter_green_score, env_nomination) = if carbon_footprint < echelle {
            ("A", "nominations.page.A")
        } else if carbon_footprint < 2.0 * echelle {
            ("B", "nominations.page.B")
        } else if carbon_footprint < 3.0 * echelle {
            ("C", "nominations.page.C")
        } else if carbon_footprint < 4.0 * echelle {
            ("D", "nominations.page.D")
        } else if carbon_footprint < 5.0 * echelle {
            ("E", "nominations.page.E")
        } else if carbon_footprint < 6.0 * echelle {
            ("F", "nominations.page.F")
        } else {
            ("G", "nominations.page.G")
        };

        (letter_green_score.to_string(), env_nomination.to_string())
    } else {
        ("N/A".to_string(), "N/A".to_string())
    }
}

pub async fn organizations_global_average_carbon_footprint(pool: &MySqlPool) -> Result<f64, sqlx::Error> {
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

pub async fn organizations_least_carbon_footprint(pool: &MySqlPool) -> Result<f64, sqlx::Error> {
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
        .fetch_optional(pool) // Changed to fetch_optional
        .await?;

    Ok(row.map(|r| r.0).unwrap_or(0.0))
}

pub async fn users_global_average_carbon_footprint(pool: &MySqlPool) -> Result<Option<f64>, sqlx::Error> {
    let row = sqlx::query_scalar::<_, Option<f64>>(
        "SELECT AVG(total_carbon_footprint) AS averageConsumption
        FROM `user`
        WHERE total_carbon_footprint IS NOT NULL
        AND total_carbon_footprint > 0;",
    )
        .fetch_one(pool)
        .await?;

    Ok(row)
}

pub async fn users_least_carbon_footprint(pool: &MySqlPool) -> Result<Option<f64>, sqlx::Error> {
    let row = sqlx::query_scalar::<_, Option<f64>>(
        "SELECT MIN(total_carbon_footprint) AS leastConsumption
        FROM `user`
        WHERE total_carbon_footprint IS NOT NULL
        AND total_carbon_footprint > 0;",
    )
        .fetch_one(pool)
        .await?;

    Ok(row)
}
