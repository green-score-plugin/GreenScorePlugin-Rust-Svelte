use axum::extract::State;
use axum::Json;
use serde::Serialize;
use sqlx::MySqlPool;
use tower_sessions::Session;
use crate::controllers::helpers::{advice, equivalent};
use crate::controllers::helpers::Equivalent;
use crate::green_score::calculate_green_score;
use crate::models::Account;

#[derive(Serialize)]
pub struct MyOrganizationInfos {
    average_daily_carbon_footprint: f64,
    equivalent: Option<Equivalent>,
    members: Vec<i32>,
    total_consumption: f64,
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

async fn average_daily_carbon_footprint(State(pool): State<MySqlPool>, org_id: i64) -> Option<f64> {
    let result = sqlx::query_as::<_, (f64,)>(
        "SELECT ROUND(
                COALESCE(
                    SUM(mw.carbon_footprint) / NULLIF(DATEDIFF(CURDATE(), MIN(DATE(mw.creation_date))) + 1, 0)
                , 0)
            , 2) AS average_daily_carbon_footprint
        FROM monitored_website mw
        JOIN user u ON u.id = mw.user_id
        WHERE u.organisation_id = ?"
    )
        .bind(org_id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok((average_daily_carbon_footprint,)) => Some(average_daily_carbon_footprint),
        Err(_) => None,
    }
}

async fn organization_members(State(pool): State<MySqlPool>, org_id: i64) -> Vec<i32> {
    let result = sqlx::query_as::<_, (i32,)>(
        "SELECT id FROM user WHERE organisation_id = ?",
    )
        .bind(org_id)
        .fetch_all(&pool)
        .await;

    match result {
        Ok(rows) => rows.into_iter().map(|(id,)| id).collect(),
        Err(_) => vec![],
    }
}

async fn total_organization_consumption(State(pool): State<MySqlPool>, org_id: i64) -> Option<f64> {
    let result = sqlx::query_as::<_, (f64,)>(
        "SELECT SUM(mw.carbon_footprint) as total_consumption
        FROM monitored_website mw
        JOIN user u
       	ON mw.user_id = u.id
        WHERE u.organisation_id = ?",
    )
        .bind(org_id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok((total_consumption,)) => Some(total_consumption),
        Err(_) => None,
    }
}

async fn organization_informations(State(pool): State<MySqlPool>, session: Session) -> Option<MyOrganizationInfos> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);

    if let Some(account) = account {
        let org_id: i64 = match account.organization_id(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return None,
            Err(_) => return None,
        };

        let average_daily_carbon_footprint_result = average_daily_carbon_footprint(State(pool.clone()), org_id).await;
        let total_consumption = total_organization_consumption(State(pool.clone()), org_id).await.unwrap_or(0.0);

        match average_daily_carbon_footprint_result {
            Some(average_daily_carbon_footprint) => {
                let equivalent = equivalent(&pool, average_daily_carbon_footprint).await;
                let members = organization_members(State(pool.clone()), org_id).await;

                Some(MyOrganizationInfos {
                    average_daily_carbon_footprint,
                    equivalent,
                    members,
                    total_consumption
                })
            }
            None => None,
        }
    } else {
        None
    }
}

pub async fn mo(State(pool): State<MySqlPool>, session: Session) -> Json<MyOrganizationResponse> {
    let organization_informations: Option<MyOrganizationInfos> = organization_informations(State(pool.clone()), session).await;

    let advices: Vec<String> = {
        let mut v = Vec::new();
        v.push(advice(&pool, false).await);
        v.push(advice(&pool, true).await);
        v
    };

    let (letter, env_nomination, equivalents) = if let Some(ref infos) = organization_informations {
        let (l, n) = calculate_green_score(&State(pool.clone()), infos.average_daily_carbon_footprint, "mo".to_string()).await;

        let mut collected: Vec<Equivalent> = Vec::new();
        for _ in 0..2 {
            if let Some(e) = equivalent(&pool, infos.total_consumption).await {
                collected.push(e);
            }
        }
        let eqs = if collected.is_empty() { None } else { Some(collected) };

        (Some(l), Some(n), eqs)
    } else {
        (None, None, None)
    };

    Json(MyOrganizationResponse {
        success: true,
        mo_infos: organization_informations,
        advices,
        letter,
        env_nomination,
        equivalents,
    })
}