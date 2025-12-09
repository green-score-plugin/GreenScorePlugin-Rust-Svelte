use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Deserialize)]
pub struct AdviceQuery {
    is_dev: String,
}

#[derive(Serialize)]
pub struct AdviceResponse {
    success: bool,
    advice: String,
}

pub async fn advice(State(pool): State<MySqlPool>, Query(params): Query<AdviceQuery>) -> Json<AdviceResponse> {
    let is_dev = params.is_dev == "true";

    let result = sqlx::query_as::<_, (String,)>(
        "SELECT advice FROM advice WHERE is_dev = ? ORDER BY RAND() LIMIT 1",
    )
    .bind(is_dev)
    .fetch_one(&pool)
    .await;

    let advice_text = match result {
        Ok((description,)) => description,
        Err(_) => {
            let fallback = if is_dev {
                "Utilisez des requêtes SQL optimisées pour réduire la charge serveur."
            } else {
                "Fermez les onglets inutilisés pour réduire la consommation d'énergie."
            };
            fallback.to_string()
        }
    };

    let response = AdviceResponse {
        success: true,
        advice: advice_text.to_string(),
    };

    Json(response)
}