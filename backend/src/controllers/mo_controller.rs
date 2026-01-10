use axum::extract::State;
use axum::Json;
use serde::Serialize;
use serde_json::Value;
use sqlx::MySqlPool;
use tower_sessions::Session;

#[derive(Serialize)]
pub struct MyOrganizationInfos {
    carbon_footprint: f64,
    members: Vec<i32>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct Equivalent {
    name: String,
    value: f64,
    icon: String,
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

pub async fn mo(State(pool): State<MySqlPool>, session: Session) -> Json<Value> {

}