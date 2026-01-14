use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::controllers::lpc_controller::{Equivalent, LastPageConsultedResponse};
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
    equivalents: Option<Vec<Equivalent>>
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

pub(crate) async fn equivalents(State(pool): State<MySqlPool>, carbon_footprint: f64) -> Vec<Equivalent> {
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



pub async fn my_data(
    State(pool): State<MySqlPool>,
    session: Session,
)-> Json<MyDataResponse> {

    let my_average_daily_carbon_footprint = get_my_average_daily_carbon_footprint(&pool, session.clone()).await;
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

    let (letter_green_score, env_nomination, equivalents) = if let Some(avg) = my_average_daily_carbon_footprint {
        let (l, n) = calculate_green_score(avg);
        let eq = crate::controllers::lpc_controller::equivalents(State(pool.clone()), avg).await;
        (Some(l), Some(n), Some(eq))
    } else {
        (None, None, None)
    };

    let total_consumption = get_total_consumption(&pool, session.clone()).await;

    Json(MyDataResponse {
        success: true,
        my_average_daily_carbon_footprint,
        average_daily_carbon_footprint,
        message_average_footprint,
        total_consumption,
        letter_green_score,
        env_nomination,
        equivalents,
    })
}
