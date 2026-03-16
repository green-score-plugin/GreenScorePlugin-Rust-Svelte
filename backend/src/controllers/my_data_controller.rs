use axum::extract::State;
use axum::Json;
use serde::{Serialize};
use sqlx::{Error, MySqlPool};
use tower_sessions::Session;
use crate::controllers::mo_controller::MyOrganizationResponse;
use crate::green_score::calculate_green_score;
use crate::service::monitored_website_service::MonitoredWebsiteService;
use crate::dto::top_polluting_site_dto::TopPollutingSite;
use crate::dto::consumption_data_point_dto::ConsumptionDataPoint;
use crate::dto::user_full::UserFull;
use crate::models::equivalent::Equivalent;
use crate::service::advice_service::AdviceService;
use crate::service::equivalent_service::EquivalentService;

#[derive(Serialize)]
pub struct MyDataResponse {
    pub success: bool,
    pub my_average_daily_carbon_footprint: Option<f64>,
    pub average_daily_carbon_footprint: Option<f64>,
    pub message_average_footprint: Option<String>,
    pub total_consumption: Option<f64>,
    pub letter_green_score: Option<String>,
    pub env_nomination: Option<String>,
    pub equivalents: Option<Vec<Equivalent>>,
    pub daily_consumption: Vec<ConsumptionDataPoint>,
    pub weekly_consumption: Vec<ConsumptionDataPoint>,
    pub monthly_consumption: Vec<ConsumptionDataPoint>,
    pub top_polluting_sites: Vec<TopPollutingSite>,
    pub advices: Vec<String>,
}

pub async fn get_top5_polluting_sites(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<TopPollutingSite>, Error> {
    MonitoredWebsiteService::get_top5_polluting_sites_by_user(pool, user_id).await
}



pub async fn get_daily_consumption(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<ConsumptionDataPoint>, sqlx::Error> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT
            DATE_FORMAT(creation_date, '%d/%m') as day,
            SUM(carbon_footprint) as total
         FROM monitored_website
         WHERE user_id = ?
         AND creation_date >= DATE_SUB(NOW(), INTERVAL 7 DAY)
         GROUP BY DATE(creation_date), day
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

pub async fn get_weekly_consumption(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Vec<ConsumptionDataPoint>, sqlx::Error> {
    let results = sqlx::query_as::<_, (i32, i32, f64)>(
        "SELECT
            CAST(YEAR(creation_date) AS SIGNED) as year,
            CAST(WEEK(creation_date, 1) AS SIGNED) as week,
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
        .map(|(_, week, value)| ConsumptionDataPoint {
            label: format!("S{}", week),
            value: (value * 100.0).round() / 100.0
        })
        .collect())
}

pub async fn get_monthly_consumption(
    pool: &MySqlPool,
    user_id: i64,
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


pub async fn get_my_average_daily_carbon_footprint(
    pool: &MySqlPool,
    session: Session
) -> Option<f64> {
    MonitoredWebsiteService::get_my_average_daily_carbon_footprint(pool, user_id).await
}

pub async fn get_average_daily_carbon_footprint(
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

pub async fn get_total_consumption(
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

    let account: Option<Account> = session.get("account").await.unwrap_or(None);

    let my_average_daily_carbon_footprint = get_my_average_daily_carbon_footprint(&pool, session.clone()).await;
    let average_daily_carbon_footprint = get_average_daily_carbon_footprint(&pool).await;
    let message_average_footprint = match (my_average_daily_carbon_footprint, average_daily_carbon_footprint) {
        (Some(user_avg), Some(global_avg)) => {
            if user_avg < global_avg * 0.8 {
                Some("widgets.common.average_daily_footprint.message.low".to_string())
            } else if user_avg <= global_avg * 1.2 {
                Some("widgets.common.average_daily_footprint.message.average".to_string())
            } else {
                Some("widgets.common.average_daily_footprint.message.high".to_string())
            }
        }
        _ => None,
    };

    let (letter_green_score, env_nomination, equivalents) = if let Some(avg) = my_average_daily_carbon_footprint {
        let (l, n) = calculate_green_score(Some(&pool), avg, "my_data".to_string()).await;
        let eqs = equivalent(&pool, avg, 2,account.as_ref()).await;
        let eqs = match eqs {
            Some(v) if !v.is_empty() => Some(v),
            _ => None,
        };

        (Some(l), Some(n), eqs)
    } else {
        (None, None, None)
    };

    let (letter_green_score, env_nomination) = calculate_green_score(&pool, my_average_daily_carbon_footprint.unwrap(), "my_data".to_string()).await;
    let equivalents: Vec<Equivalent> = EquivalentService::equivalent(&pool, Some(user_id), 2, my_average_daily_carbon_footprint.unwrap()).await.unwrap_or_default();

    let total_consumption = get_total_consumption(&pool, user_id).await;

    let daily_consumption = get_daily_consumption(&pool, user_id).await.unwrap_or_default();
    let weekly_consumption = get_weekly_consumption(&pool, user_id).await.unwrap_or_default();
    let monthly_consumption = get_monthly_consumption(&pool, user_id).await.unwrap_or_default();

let advices: Vec<String> = vec![
    AdviceService::get_one_random_advice(&pool, false).await.unwrap_or_default(),
    AdviceService::get_one_random_advice(&pool, true).await.unwrap_or_default(),
];

    let top_polluting_sites = get_top5_polluting_sites(&pool, user_id).await.unwrap_or_default();

    Json(MyDataResponse {
        success: true,
        my_average_daily_carbon_footprint,
        average_daily_carbon_footprint,
        message_average_footprint,
        total_consumption,
        letter_green_score: Some(letter_green_score),
        env_nomination: Some(env_nomination),
        equivalents: Some(equivalents),
        daily_consumption,
        weekly_consumption,
        monthly_consumption,
        top_polluting_sites,
        advices,
    })
}

fn error_response() -> Json<MyDataResponse> {
    Json(MyDataResponse {
        success: false,
        my_average_daily_carbon_footprint: None,
        average_daily_carbon_footprint: None,
        message_average_footprint: None,
        total_consumption: None,
        letter_green_score: None,
        env_nomination: None,
        equivalents: None,
        daily_consumption: vec![],
        weekly_consumption: vec![],
        monthly_consumption: vec![],
        top_polluting_sites: vec![],
        advices: vec![],
    })
}