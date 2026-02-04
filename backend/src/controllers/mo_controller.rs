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
    pub name: String,
    average_daily_carbon_footprint: f64,
    equivalent: Option<Equivalent>,
    pub total_consumption: f64,
}

#[derive(Serialize)]
pub struct MyOrganizationResponse {
    pub success: bool,
    pub mo_infos: Option<MyOrganizationInfos>,
    pub advices: Vec<String>,
    pub letter: Option<String>,
    pub env_nomination: Option<String>,
    pub equivalents: Option<Vec<Equivalent>>,
    pub daily_consumption: Vec<ConsumptionDataPoint>,
    pub weekly_consumption: Vec<ConsumptionDataPoint>,
    pub monthly_consumption: Vec<ConsumptionDataPoint>,
    pub top_polluting_sites: Vec<TopPollutingSite>,
}

#[derive(Serialize)]
pub struct ConsumptionDataPoint {
    label: String,
    value: f64,
}

#[derive(Serialize)]
pub struct TopPollutingSite {
    pub url_domain: String,
    total_footprint: f64,
}

async fn get_top5_polluting_sites(pool: &MySqlPool, org_id: i64) -> Vec<TopPollutingSite> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT mw.url_domain, SUM(mw.carbon_footprint) as total_footprint FROM monitored_website mw JOIN user u ON u.id = mw.user_id WHERE u.organisation_id = ? AND mw.url_domain IS NOT NULL GROUP BY mw.url_domain ORDER BY total_footprint DESC LIMIT 5"
    )
        .bind(org_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    results.into_iter()
        .map(|(url_domain, total_footprint)| TopPollutingSite {
            url_domain,
            total_footprint: (total_footprint * 100.0).round() / 100.0
        })
        .collect()
}

async fn average_daily_carbon_footprint(pool: &MySqlPool, org_id: i64) -> f64 {
    sqlx::query_as::<_, (f64,)>(
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
        .fetch_one(pool)
        .await
        .map(|(val,)| val)
        .unwrap_or(0.0)
}

async fn total_organization_consumption(pool: &MySqlPool, org_id: i64) -> Option<f64> {
    sqlx::query_as::<_, (f64,)>(
        "SELECT SUM(mw.carbon_footprint) as total_consumption
        FROM monitored_website mw
        JOIN user u
        ON mw.user_id = u.id
        WHERE u.organisation_id = ?",
    )
        .bind(org_id)
        .fetch_one(pool)
        .await
        .ok()
        .map(|(val,)| val)
}

async fn organization_name(pool: &MySqlPool, org_id: i64) -> Option<String> {
    sqlx::query_as::<_, (String,)>(
        "SELECT organisation_name FROM organisation WHERE id = ? LIMIT 1",
    )
        .bind(org_id)
        .fetch_one(pool)
        .await
        .ok()
        .map(|(name,)| name)
}

// CORRECTION ICI : Retourne Option pour gérer le cas "phantom organization"
async fn organization_informations(pool: &MySqlPool, org_id: i64, account: &Account) -> Option<MyOrganizationInfos> {
    // Si l'organisation n'a pas de nom (n'existe pas), on doit renvoyer None pour satisfaire le test.
    let name = organization_name(pool, org_id).await?;

    // Le reste est robuste (valeurs par défaut 0.0)
    let average_daily_carbon_footprint = average_daily_carbon_footprint(pool, org_id).await;
    let total_consumption = total_organization_consumption(pool, org_id).await.unwrap_or(0.0);

    let equivalent_vec = equivalent(pool, average_daily_carbon_footprint, 1, Some(account)).await;
    let equivalent = equivalent_vec.and_then(|mut v| v.pop());

    Some(MyOrganizationInfos {
        name,
        average_daily_carbon_footprint,
        equivalent,
        total_consumption,
    })
}

async fn get_daily_consumption(pool: &MySqlPool, org_id: i64) -> Vec<ConsumptionDataPoint> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT DATE_FORMAT(mw.creation_date, '%d/%m') as day, SUM(mw.carbon_footprint) as total FROM monitored_website mw JOIN user u ON u.id = mw.user_id WHERE u.organisation_id = ? AND mw.creation_date >= DATE_SUB(NOW(), INTERVAL 7 DAY) GROUP BY DATE(mw.creation_date) ORDER BY DATE(mw.creation_date) ASC"
    )
        .bind(org_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    results.into_iter()
        .map(|(label, value)| ConsumptionDataPoint { label, value: (value * 100.0).round() / 100.0 })
        .collect()
}

async fn get_weekly_consumption(pool: &MySqlPool, org_id: i64) -> Vec<ConsumptionDataPoint> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT CONCAT('Semaine ', WEEK(mw.creation_date, 1)) as week, SUM(mw.carbon_footprint) as total FROM monitored_website mw JOIN user u ON u.id = mw.user_id WHERE u.organisation_id = ? AND mw.creation_date >= DATE_SUB(NOW(), INTERVAL 4 WEEK) GROUP BY WEEK(mw.creation_date, 1) ORDER BY WEEK(mw.creation_date, 1) ASC"
    )
        .bind(org_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    results.into_iter()
        .map(|(label, value)| ConsumptionDataPoint { label, value: (value * 100.0).round() / 100.0 })
        .collect()
}

async fn get_monthly_consumption(pool: &MySqlPool, org_id: i64) -> Vec<ConsumptionDataPoint> {
    let results = sqlx::query_as::<_, (String, f64)>(
        "SELECT DATE_FORMAT(mw.creation_date, '%m/%Y') as month, SUM(mw.carbon_footprint) as total FROM monitored_website mw JOIN user u ON u.id = mw.user_id WHERE u.organisation_id = ? AND mw.creation_date >= DATE_SUB(NOW(), INTERVAL 12 MONTH) GROUP BY MONTH(mw.creation_date), YEAR(mw.creation_date) ORDER BY YEAR(mw.creation_date), MONTH(mw.creation_date) ASC"
    )
        .bind(org_id)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

    results.into_iter()
        .map(|(label, value)| ConsumptionDataPoint { label, value: (value * 100.0).round() / 100.0 })
        .collect()
}


pub async fn mo(State(pool): State<MySqlPool>, session: Session) -> Json<MyOrganizationResponse> {
    let account: Option<Account> = session.get("account").await.unwrap_or(None);
    let account_ref = account.clone();

    let org_id: i64 = match account {
        Some(acc) => match acc.organization_id(&pool).await {
            Ok(Some(id)) => id,
            Ok(None) | Err(_) => return error_response(),
        },
        None => return error_response(),
    };

    // CORRECTION ICI : On récupère une Option et on l'utilise telle quelle
    let organization_informations = organization_informations(&pool, org_id, account_ref.as_ref().unwrap()).await;

    let advices: Vec<String> = {
        let mut v = Vec::new();
        v.push(advice(&pool, false).await);
        v.push(advice(&pool, true).await);
        v
    };

    let (letter, env_nomination, equivalents) = if let Some(ref infos_ref) = organization_informations {
        let (l, n) = calculate_green_score(Some(&pool), infos_ref.average_daily_carbon_footprint, "mo".to_string()).await;
        // Le calcul des équivalents utilise total_consumption qui est safe (0.0 au pire)
        let eqs = equivalent(&pool, infos_ref.total_consumption, 2, account_ref.as_ref()).await;
        let eqs_filtered = match eqs {
            Some(v) if !v.is_empty() => Some(v),
            _ => None,
        };
        (Some(l), Some(n), eqs_filtered)
    } else {
        (None, None, None)
    };

    let daily_consumption = get_daily_consumption(&pool, org_id).await;
    let weekly_consumption = get_weekly_consumption(&pool, org_id).await;
    let monthly_consumption = get_monthly_consumption(&pool, org_id).await;
    let top_polluting_sites = get_top5_polluting_sites(&pool, org_id).await;

    Json(MyOrganizationResponse {
        success: true,
        mo_infos: organization_informations,
        advices,
        letter,
        env_nomination,
        equivalents,
        daily_consumption,
        weekly_consumption,
        monthly_consumption,
        top_polluting_sites
    })
}

fn error_response() -> Json<MyOrganizationResponse> {
    Json(MyOrganizationResponse {
        success: false,
        mo_infos: None,
        advices: vec![],
        letter: None,
        env_nomination: None,
        equivalents: None,
        daily_consumption: vec![],
        weekly_consumption: vec![],
        monthly_consumption: vec![],
        top_polluting_sites: vec![],
    })
}
