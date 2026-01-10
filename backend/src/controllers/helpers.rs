use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize, sqlx::FromRow)]
pub struct Equivalent {
    name: String,
    value: f64,
    icon: String,
}

pub async fn advices(pool: &MySqlPool) -> Vec<String> {
    use tokio::try_join;

    let dev_query = sqlx::query_as::<_, (String,)>(
        "SELECT advice FROM advice WHERE is_dev = 1 ORDER BY RAND() LIMIT 1",
    )
        .fetch_one(pool);

    let non_dev_query = sqlx::query_as::<_, (String,)>(
        "SELECT advice FROM advice WHERE is_dev = 0 ORDER BY RAND() LIMIT 1",
    )
        .fetch_one(pool);

    match try_join!(dev_query, non_dev_query) {
        Ok(((dev_advice,), (non_dev_advice,))) => vec![dev_advice, non_dev_advice],
        Err(_) => vec![
            "Utilisez des requêtes SQL optimisées pour réduire la charge serveur.".to_string(),
            "Fermez les onglets inutilisés pour réduire la consommation d'énergie.".to_string(),
        ],
    }
}
pub async fn equivalents(pool: &MySqlPool, carbon_footprint: f64) -> Vec<Equivalent> {
    let carbon_footprint_in_kg = carbon_footprint / 1000.0;

    let equivalents = sqlx::query_as::<_, Equivalent>(
        "SELECT name, ROUND(? * equivalent, 2) as value, icon_thumbnail as icon
         FROM equivalent
         WHERE (? * equivalent) >= 1.0
         ORDER BY RAND()
         LIMIT 2",
    )
        .bind(carbon_footprint_in_kg)
        .bind(carbon_footprint_in_kg)
        .fetch_all(pool)
        .await;

    match equivalents {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Erreur SQL : {:?}", e);
            vec![]
        },
    }
}