use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::models::Account;

#[derive(Deserialize)]
pub struct AdviceQuery {
    is_dev: String,
}

#[derive(Serialize)]
pub struct AdviceResponse {
    success: bool,
    advice: String,
}

#[derive(Serialize)]
pub struct LastLinkResponse {
    success: bool,
    link: String,
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

pub async fn last_link(State(pool): State<MySqlPool>, session: Session) -> Json<LastLinkResponse> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);

    if let Some(account) = account {
        let id = account.id();
        eprintln!("🔍 Recherche pour user_id: {}", id);

        let result = sqlx::query_as::<_, (String,)>(
            "SELECT url_full FROM monitored_website WHERE user_id = ? ORDER BY creation_date DESC LIMIT 1",
        )
        .bind(id)
        .fetch_one(&pool)
        .await;

        let link_text = match result {
            Ok((link,)) => {
                eprintln!("✅ Lien trouvé: {}", link);
                link
            },
            Err(e) => {
                eprintln!("❌ Erreur DB: {:?}", e);
                "https://greenscoreweb.example.com".to_string()
            },
        };

        let response = LastLinkResponse {
            success: true,
            link: link_text.to_string(),
        };

        return Json(response);
    }

    Json(LastLinkResponse {
        success: false,
        link: String::new(),
    })
}