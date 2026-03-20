use axum::Json;
use axum::extract::State;
use sqlx::MySqlPool;
use tower_sessions::Session;
use serde::{Deserialize, Serialize};
use crate::models::user::User;
use crate::dto::user_full::UserFull;
use crate::service::auth_service::AuthService;
use crate::service::service_service::ServiceService;
use crate::error::AppError;
use crate::dto::current_account_response::CurrentAccountResponse;

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
pub struct ServiceCreationRequest {
    pub service_name: String,
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


pub async fn login(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<LoginRequest>) -> Result<Json<GenericResponse>, AppError> {

    let email = payload.email.trim();
    let password = payload.password.trim();

    let user_full = AuthService::login(&pool, email, password).await?;

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

pub async fn create_service(session: Session, State(pool): State<MySqlPool>, Json(payload): Json<ServiceCreationRequest>) -> Result<Json<CurrentAccountResponse>, AppError>
{
    let service_name = payload.service_name.trim();

    let mut user_full = session.get::<UserFull>("user_full").await
        .map_err(|_| AppError::InternalServerError("Session error".to_string()))?
        .ok_or(AppError::AuthError("errors.auth.not_logged_in".to_string()))?;

    let organisation_id = user_full.organisation.as_ref()
        .map(|org| org.id)
        .ok_or(AppError::BadRequest("User must be in an organization to create a service".to_string()))?;

    let services = ServiceService::create_service(&pool, organisation_id, service_name).await
        .map_err(AppError::BadRequest)?;

    if let Some(service) = services.iter().find(|s| s.nom == service_name) {
        user_full.service = Some(service.clone());
    }

    session.insert("user_full", user_full.clone()).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(CurrentAccountResponse {
        success: true,
        user_full: Some(user_full),
        services: Some(services),
        message: None,
    }))
}

pub async fn get_current_account(session: Session, State(pool): State<MySqlPool>) -> Result<Json<CurrentAccountResponse>, AppError> {

    let user_full_opt: Option<UserFull> = session.get("user_full").await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    if let Some(user_full) = user_full_opt {
        let mut services = None;
        if let Some(org) = &user_full.organisation {
             if let Ok(s) = ServiceService::get_organisation_services(&pool, org.id).await {
                 services = Some(s);
             }
        }

        Ok(Json(CurrentAccountResponse {
            success: true,
            user_full: Some(user_full),
            services,
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