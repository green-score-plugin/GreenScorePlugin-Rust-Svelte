use axum::extract::{State};
use axum::Json;
use serde::{Serialize};
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::models::Account;
use crate::green_score::calculate_green_score;

#[derive(Serialize)]
pub struct LastPageConsultedInfos {
    link: String,
    queries_quantity: i32,
    carbon_footprint: f64,
    data_transferred: f64,
    loading_time: f64,
    country: String,
}
#[derive(Serialize)]
pub struct LastPageConsultedResponse {
    success: bool,
    lpc_infos: Option<LastPageConsultedInfos>,
    advices: Vec<String>,
    letter: Option<String>,
    env_nomination: Option<String>,
}

async fn last_search_informations(State(pool): State<MySqlPool>, session: Session) -> Option<LastPageConsultedInfos> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);

    if let Some(account) = account {
        let id = account.id();

        let result = sqlx::query_as::<_, (String, i32, f64, f64, f64, String)>(
            "SELECT url_full, queries_quantity, carbon_footprint, data_transferred, loading_time, country FROM monitored_website WHERE user_id = ? ORDER BY creation_date DESC LIMIT 1",
        )
            .bind(id)
            .fetch_one(&pool)
            .await;

        match result {
            Ok((url_full, queries_quantity, carbon_footprint, data_transferred, loading_time, country)) => {
                Some(LastPageConsultedInfos {
                    link: url_full,
                    queries_quantity,
                    carbon_footprint,
                    data_transferred,
                    loading_time,
                    country,
                })
            }
            Err(_) => None,
        }
    } else {
        None
    }
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
pub async fn lpc(State(pool): State<MySqlPool>, session: Session) -> Json<LastPageConsultedResponse> {
    let last_search_informations: Option<LastPageConsultedInfos> = last_search_informations(State(pool.clone()), session).await;

    let advices: Vec<String> = advices(State(pool.clone())).await;

    let (letter, env_nomination) = if let Some(ref infos) = last_search_informations {
        let (l, n) = calculate_green_score(infos.carbon_footprint);
        (Some(l), Some(n))
    } else {
        (None, None)
    };

    Json(LastPageConsultedResponse {
        success: true,
        lpc_infos: last_search_informations,
        advices,
        letter,
        env_nomination,
    })
}