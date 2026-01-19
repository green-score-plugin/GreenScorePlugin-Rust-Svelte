use axum::extract::{State, Query};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::models::Account;
use crate::green_score::calculate_green_score;
use crate::controllers::helpers::{advice, equivalent};
use crate::controllers::helpers::Equivalent;

#[derive(Deserialize)]
pub struct LpcParams {
    url_full: Option<String>,
    country: Option<String>,
    #[serde(rename = "totalConsu")]
    total_consu: Option<f64>,
    #[serde(rename = "pageSize")]
    page_size: Option<f64>,
    #[serde(rename = "loadingTime")]
    loading_time: Option<f64>,
    #[serde(rename = "queriesQuantity")]
    queries_quantity: Option<i32>,
}

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
    equivalents: Option<Vec<Equivalent>>,
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

pub async fn lpc(
    session: Session,
    State(pool): State<MySqlPool>,
    Query(params): Query<LpcParams>,
) -> Json<LastPageConsultedResponse> {
    let last_search_informations: Option<LastPageConsultedInfos> = if let (
        Some(url),
        Some(country),
        Some(carbon),
        Some(data),
        Some(time),
        Some(queries),
    ) = (
        params.url_full,
        params.country,
        params.total_consu,
        params.page_size,
        params.loading_time,
        params.queries_quantity,
    ) {
        Some(LastPageConsultedInfos {
            link: url,
            country,
            carbon_footprint: carbon,
            data_transferred: data,
            loading_time: time,
            queries_quantity: queries,
        })
    } else {
        last_search_informations(State(pool.clone()), session).await
    };

    let advices: Vec<String> = {
        let mut v = Vec::new();
        v.push(advice(&pool, false).await);
        v.push(advice(&pool, true).await);
        v
    };

    let (letter, env_nomination, equivalents) = if let Some(ref infos) = last_search_informations {
        let (l, n) = calculate_green_score(&State(pool.clone()), infos.carbon_footprint, "lpc".to_string()).await;
        let mut collected: Vec<Equivalent> = Vec::new();
        for _ in 0..2 {
            if let Some(e) = equivalent(&pool, infos.carbon_footprint).await {
                collected.push(e);
            }
        }
        let eqs = if collected.is_empty() { None } else { Some(collected) };

        (Some(l), Some(n), eqs)
    } else {
        (None, None, None)
    };


    Json(LastPageConsultedResponse {
        success: true,
        lpc_infos: last_search_informations,
        advices,
        letter,
        env_nomination,
        equivalents,
    })
}
