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

async fn organization_informations(State(pool): State<MySqlPool>, session: Session) -> Option<MyOrganizationInfos> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);

    if let Some(account) = account {
        let org_id: i64 = match account.organization_id(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) => return None,
            Err(_) => return None,
        };

        let result = sqlx::query_as::<_, (f64,)>(
            "SELECT ROUND(AVG(mw.carbon_footprint), 2) AS average_daily_carbon_footprint
            FROM monitored_website mw
            JOIN user u on u.id = mw.user_id
            WHERE u.organisation_id = ?"
        )
            .bind(org_id)
            .fetch_one(&pool)
            .await;

        match result {
            Ok((average_daily_carbon_footprint,)) => {
                let equivalent = equivalent(&pool, average_daily_carbon_footprint).await;

                let members_result = sqlx::query_as::<_, (i32,)>(
                    "SELECT id FROM user WHERE organization_id = ?",
                )
                    .bind(org_id)
                    .fetch_all(&pool)
                    .await;

                let members = match members_result {
                    Ok(rows) => rows.into_iter().map(|(id,)| id).collect(),
                    Err(_) => vec![],
                };

                Some(MyOrganizationInfos {
                    average_daily_carbon_footprint,
                    equivalent,
                    members,
                })
            }
            Err(_) => None,
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
        let (l, n) = calculate_green_score(infos.average_daily_carbon_footprint);

        let mut collected: Vec<Equivalent> = Vec::new();
        for _ in 0..2 {
            if let Some(e) = equivalent(&pool, infos.average_daily_carbon_footprint).await {
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