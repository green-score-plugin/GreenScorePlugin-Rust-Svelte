use axum::extract::State;
use axum::Json;
use serde::{Serialize};
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::controllers::helpers::{advice, equivalent};
use crate::controllers::helpers::Equivalent;
use crate::green_score::calculate_green_score;
use crate::models::Account;

#[derive(Serialize)]
pub struct MyDataResponse {
    success: bool,
    my_average_daily_carbon_footprint: Option<f64>,
    average_daily_carbon_footprint: Option<f64>,
    message_average_footprint: Option<String>,
    total_consumption: Option<f64>,
    letter_green_score: Option<String>,
    env_nomination: Option<String>,
    equivalents: Option<Vec<Equivalent>>,
    daily_consumption: Vec<ConsumptionDataPoint>,
    weekly_consumption: Vec<ConsumptionDataPoint>,
    monthly_consumption: Vec<ConsumptionDataPoint>,
    top_polluting_sites: Vec<TopPollutingSite>,
    advices: Vec<String>,
}
#[derive(Serialize, Debug)]
pub struct ConsumptionDataPoint {
    label: String,
    value: f64,
}
#[derive(Serialize, Debug)]
pub struct TopPollutingSite {
    url_domain: String,
    total_footprint: f64,
}

async fn get_top5_polluting_sites(
    pool: &MySqlPool,
    user_id: i32,
) -> Result<Vec<TopPollutingSite>, sqlx::Error> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT
            url_domain,
            SUM(carbon_footprint) as total_footprint
         FROM monitored_website
         WHERE user_id = ?
         AND url_domain IS NOT NULL
         GROUP BY url_domain
         ORDER BY total_footprint DESC
         LIMIT 5"
    )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(results.into_iter()
        .map(|(url_domain, total_footprint)| TopPollutingSite {
            url_domain,
            total_footprint: (total_footprint * 100.0).round() / 100.0
        })
        .collect())
}



async fn get_daily_consumption(
    pool: &MySqlPool,
    user_id: i32,
) -> Result<Vec<ConsumptionDataPoint>, sqlx::Error> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT
            DATE_FORMAT(creation_date, '%d/%m') as day,
            SUM(carbon_footprint) as total
         FROM monitored_website
         WHERE user_id = ?
         AND creation_date >= DATE_SUB(NOW(), INTERVAL 7 DAY)
         GROUP BY DATE(creation_date)
         ORDER BY DATE(creation_date) ASC"
    )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(results.into_iter()
        .map(|(label, value)| ConsumptionDataPoint {
            label,
            value: (value * 100.0).round() / 100.0
        })
        .collect())
}

async fn get_weekly_consumption(
    pool: &MySqlPool,
    user_id: i32,
) -> Result<Vec<ConsumptionDataPoint>, sqlx::Error> {
    let results = sqlx::query_as::<_, (i32, f64)>(
        "SELECT
            YEAR(creation_date) as year,
            WEEK(creation_date, 1) as week,
            SUM(carbon_footprint) as total
         FROM monitored_website
         WHERE user_id = ?
         AND creation_date >= DATE_SUB(NOW(), INTERVAL 4 WEEK)
         GROUP BY year, week
         ORDER BY year, week ASC"
    )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(results.into_iter()
        .map(|(week, value)| ConsumptionDataPoint {
            label: format!("S{}", week),
            value: (value * 100.0).round() / 100.0
        })
        .collect())
}

async fn get_monthly_consumption(
    pool: &MySqlPool,
    user_id: i32,
) -> Result<Vec<ConsumptionDataPoint>, sqlx::Error> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT
            DATE_FORMAT(creation_date, '%m/%Y') as month,
            SUM(carbon_footprint) as total
         FROM monitored_website
         WHERE user_id = ?
         AND creation_date >= DATE_SUB(NOW(), INTERVAL 12 MONTH)
         GROUP BY YEAR(creation_date), MONTH(creation_date)
         ORDER BY YEAR(creation_date), MONTH(creation_date) ASC"
    )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

    Ok(results.into_iter()
        .map(|(label, value)| ConsumptionDataPoint {
            label,
            value: (value * 100.0).round() / 100.0
        })
        .collect())
}


async fn get_my_average_daily_carbon_footprint(
    pool: &MySqlPool,
    session: Session
) -> Option<f64> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);
    if let Some(account) = account {
        let user_id = account.id();
        let result = sqlx::query_as::<_, (String, f64)>(
            "SELECT CAST(DATE(creation_date) AS CHAR) as day, AVG(carbon_footprint) as daily_average
     FROM monitored_website
     WHERE user_id = ?
     GROUP BY day"
        )
            .bind(user_id)
            .fetch_all(pool)
            .await;

        match result {
            Ok(daily_averages) if !daily_averages.is_empty() => {
                let sum: f64 = daily_averages.iter().map(|(_, avg)| avg).sum();
                let average = sum / daily_averages.len() as f64;
                Some((average * 100.0).round() / 100.0)
            }
            _ => None,
        }
    } else {
        None
    }
}

async fn get_average_daily_carbon_footprint(
    pool: &MySqlPool,
) -> Option<f64> {
    let result = sqlx::query_as::<_, (String, f64)>(
        "SELECT
            CAST(DATE(creation_date) AS CHAR) as day,
            AVG(carbon_footprint) as daily_average
         FROM monitored_website
         GROUP BY day"
    )
        .fetch_all(pool)
        .await;

    match result {
        Ok(daily_averages) if !daily_averages.is_empty() => {
            let sum: f64 = daily_averages.iter().map(|(_, avg)| avg).sum();
            let average = sum / daily_averages.len() as f64;
            Some((average * 100.0).round() / 100.0)
        }
        _ => None,
    }
}

async fn get_total_consumption(
    pool: &MySqlPool,
    session: Session
) -> Option<f64> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);
    if let Some(account) = account {
        let user_id = account.id();
        let result = sqlx::query_scalar::<_, f64>(
            "SELECT SUM(carbon_footprint) FROM monitored_website WHERE user_id = ?"
        )
            .bind(user_id)
            .fetch_one(pool)
            .await;

        match result {
            Ok(total) => Some((total * 100.0).round() / 100.0),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub async fn my_data(
    State(pool): State<MySqlPool>,
    session: Session,
)-> Json<MyDataResponse> {

    let my_average_daily_carbon_footprint = get_my_average_daily_carbon_footprint(&pool, session.clone()).await;
    let average_daily_carbon_footprint = get_average_daily_carbon_footprint(&pool).await;
    let message_average_footprint = match (my_average_daily_carbon_footprint, average_daily_carbon_footprint) {
        (Some(user_avg), Some(global_avg)) => {
            if user_avg < global_avg * 0.8 {
                Some("Votre empreinte carbone quotidienne moyenne est faible par rapport à la moyenne globale.".to_string())
            } else if user_avg <= global_avg * 1.2 {
                Some("Votre empreinte carbone quotidienne moyenne est dans la moyenne globale.".to_string())
            } else {
                Some("Votre empreinte carbone quotidienne moyenne est élevée par rapport à la moyenne globale.".to_string())
            }
        }
        _ => None,
    };

    let (letter_green_score, env_nomination, equivalents) = if let Some(avg) = my_average_daily_carbon_footprint {
        let (l, n) = calculate_green_score(&pool, avg, "my_data".to_string()).await;
        let eqs = equivalent(&pool, avg, 2).await;
        let eqs = match eqs {
            Some(v) if !v.is_empty() => Some(v),
            _ => None,
        };

        (Some(l), Some(n), eqs)
    } else {
        (None, None, None)
    };

    let total_consumption = get_total_consumption(&pool, session.clone()).await;

    let (daily_consumption, weekly_consumption, monthly_consumption) =
        if let Some(account) = session.get::<Account>("account").await.unwrap_or(None) {
            let user_id = account.id();
            let daily = get_daily_consumption(&pool, user_id as i32).await.unwrap_or_default();
            let weekly = get_weekly_consumption(&pool, user_id as i32).await.unwrap_or_default();
            let monthly = get_monthly_consumption(&pool, user_id as i32).await.unwrap_or_default();
            (daily, weekly, monthly)
        } else {
            (vec![], vec![], vec![])
        };
    let advices: Vec<String> = {
        let mut v = Vec::new();
        v.push(advice(&pool, false).await);
        v.push(advice(&pool, true).await);
        v
    };

    let top_polluting_sites = if let Some(account) = session.get::<Account>("account").await.unwrap_or(None) {
        get_top5_polluting_sites(&pool, account.id() as i32).await.unwrap_or_default()
    } else {
        vec![]
    };
    Json(MyDataResponse {
        success: true,
        my_average_daily_carbon_footprint,
        average_daily_carbon_footprint,
        message_average_footprint,
        total_consumption,
        letter_green_score,
        env_nomination,
        equivalents,
        daily_consumption,
        weekly_consumption,
        monthly_consumption,
        top_polluting_sites,
        advices,
    })
}
