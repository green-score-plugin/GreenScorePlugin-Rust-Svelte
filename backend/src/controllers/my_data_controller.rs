use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::controllers::lpc_controller::LastPageConsultedResponse;
use crate::models::Account;

#[derive(Serialize)]
pub struct MyDataResponse {
    success: bool,
    my_average_daily_carbon_footprint: Option<f64>,
    average_daily_carbon_footprint: Option<f64>,
    message_average_footprint: Option<String>,
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

        println!("Daily averages result: {:?}", result);

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

    println!("Global daily averages result: {:?}", result);

    match result {
        Ok(daily_averages) if !daily_averages.is_empty() => {
            let sum: f64 = daily_averages.iter().map(|(_, avg)| avg).sum();
            let average = sum / daily_averages.len() as f64;
            Some((average * 100.0).round() / 100.0)
        }
        _ => None,
    }
}




pub async fn my_data(
    State(pool): State<MySqlPool>,
    session: Session,
)-> Json<MyDataResponse> {

    let my_average_daily_carbon_footprint = get_my_average_daily_carbon_footprint(&pool, session).await;
    println!("My average daily carbon footprint: {:?}", my_average_daily_carbon_footprint);
    let average_daily_carbon_footprint = get_average_daily_carbon_footprint(&pool).await;
    println!("Average daily carbon footprint: {:?}", average_daily_carbon_footprint);
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

    Json(MyDataResponse {
        success: true,
        my_average_daily_carbon_footprint,
        average_daily_carbon_footprint,
        message_average_footprint,
    })
}
