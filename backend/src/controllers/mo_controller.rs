use axum::extract::State;
use axum::Json;
use serde::Serialize;
use serde_json::Value;
use sqlx::MySqlPool;
use tower_sessions::Session;

#[derive(Serialize)]
pub struct MyOrganizationInfos {
    carbon_footprint: f64,
    members: Vec<i32>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Equivalent {
    name: String,
    value: f64,
    icon: String,
}

#[derive(Serialize)]
pub struct MyOrganizationResponse {
    success: bool,
    mo_infos: Option<MyOrganizationInfos>,
    advices: Vec<String>,
    letter: Option<String>,
    env_nomination: Option<String>,
    equivalents: Option<Vec<Equivalent>>,
}

async fn advices(State(pool): State<MySqlPool>) -> Vec<String> {
    use tokio::try_join;

    let dev_query = sqlx::query_as::<_, (String,)>(
        "SELECT advice FROM advice WHERE is_dev = 1 ORDER BY RAND() LIMIT 1",
    )
        .fetch_one(&pool);

    let non_dev_query = sqlx::query_as::<_, (String,)>(
        "SELECT advice FROM advice WHERE is_dev = 0 ORDER BY RAND() LIMIT 1",
    )
        .fetch_one(&pool);

    match try_join!(dev_query, non_dev_query) {
        Ok(((dev_advice,), (non_dev_advice,))) => vec![dev_advice, non_dev_advice],
        Err(_) => vec![
            "Utilisez des requêtes SQL optimisées pour réduire la charge serveur.".to_string(),
            "Fermez les onglets inutilisés pour réduire la consommation d'énergie.".to_string(),
        ],
    }
}
async fn equivalents(State(pool): State<MySqlPool>, carbon_footprint: f64) -> Vec<Equivalent> {
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
        .fetch_all(&pool)
        .await;

    match equivalents {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("Erreur SQL : {:?}", e);
            vec![]
        },
    }
}

pub async fn mo(State(pool): State<MySqlPool>, session: Session) -> Json<Value> {

}