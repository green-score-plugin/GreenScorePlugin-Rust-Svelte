use axum::Json;
use axum::extract::State;
use sqlx::MySqlPool;
use tower_sessions::Session;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::models::organisation::Organisation;
use crate::dto::user_full::UserFull;
use crate::service::auth_service::AuthService;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct InscriptionRequest {
    pub email: String,
    pub password: String,
    pub lastname: String,
    pub firstname: String,
}

#[derive(Deserialize)]
pub struct InscriptionOrgaRequest {
    pub orga_name : String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub siret : Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>
}

#[derive(Serialize)]
pub struct CurrentAccountResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_full: Option<UserFull>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}


pub async fn login(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<LoginRequest>) -> Result<Json<GenericResponse>, AppError> {

    let email = payload.email.trim();
    let password = payload.password.trim();

    let user_full = AuthService::login(&pool, email, password).await
        .map_err(AppError::AuthError)?;

    session.insert("user_full", user_full).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(GenericResponse {
        success: true,
        message: None,
    }))
}

pub async fn inscription(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<InscriptionRequest>) -> Result<Json<GenericResponse>, AppError> {
    let email = payload.email.trim();
    let password = payload.password.trim();
    let first_name = payload.firstname.trim();
    let last_name = payload.lastname.trim();

    if email.is_empty() || password.is_empty() || first_name.is_empty() || last_name.is_empty() {
        return Err(AppError::BadRequest("errors.auth.missing_fields".to_string()));
    }

    let user_id = AuthService::inscription(&pool, email, password, first_name, last_name).await
        .map_err(AppError::BadRequest)?;

    let user = User {
        id: user_id,
        id_organisation: None,
        id_service: None,
        email: email.to_string(),
        prenom: first_name.to_string(),
        nom: last_name.to_string(),
        est_admin: false,
        total_carbon_footprint: 0.0,
    };

    let user_full = UserFull {
        user: user.clone(),
        organisation: None,
        service: None,
    };

    session.insert("user_full", user_full).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(GenericResponse {
        success: true,
        message: None,
    }))
}

pub async fn inscription_orga(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<InscriptionOrgaRequest>) -> Result<Json<CurrentAccountResponse>, AppError>
{
    let orga_name = payload.orga_name.trim();
    let siret = payload.siret.as_ref().map(|s| s.trim().to_string());


    let mut user_full = session.get::<UserFull>("user_full").await
        .map_err(|_| AppError::InternalServerError("Session error".to_string()))?
        .ok_or(AppError::AuthError("errors.auth.not_logged_in".to_string()))?;

    let user_id = user_full.user.id;

    let (organisation_id, organisation_code) = AuthService::inscription_orga(&pool, orga_name, siret.as_deref(), user_id).await
        .map_err(AppError::BadRequest)?;

    let organisation = Organisation {
        id: organisation_id,
        nom: orga_name.to_string(),
        siret,
        code: organisation_code,
    };

    user_full.organisation = Some(organisation);

    session.insert("user_full", user_full.clone()).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(CurrentAccountResponse {
        success: true,
        user_full: Some(user_full),
        message: None,
    }))
}

pub async fn get_current_account(session: Session) -> Result<Json<CurrentAccountResponse>, AppError> {

    let user_full_opt: Option<UserFull> = session.get("user_full").await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    if let Some(user_full) = user_full_opt {
        Ok(Json(CurrentAccountResponse {
            success: true,
            user_full: Some(user_full),
            message: None,
        }))
    } else {
        Err(AppError::AuthError("errors.auth.not_logged_in".to_string()))
    }
}

pub async fn logout(session: Session) -> Result<Json<GenericResponse>, AppError> {
    session.delete().await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;
    Ok(Json(GenericResponse {
        success: true,
        message: None,
    }))
}