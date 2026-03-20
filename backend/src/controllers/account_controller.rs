use axum::extract::State;
use axum::Json;
use sqlx::MySqlPool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_sessions::Session;
use crate::dto::user_full::UserFull;
use crate::service::user_service::UserService;
use crate::service::organisation_service::OrganisationService;
use crate::service::equivalent_service::EquivalentService;
use crate::dto::update_account_request_dto::UpdateAccountRequest;
use crate::dto::update_organisation_request_dto::UpdateOrganisationRequest;
use crate::dto::equivalent_dto::EquivalentSelection;
use crate::middleware::auth::AuthenticatedUser;
use crate::error::AppError;
use crate::models::organisation::Organisation;
use crate::dto::current_account_response::CurrentAccountResponse;

#[derive(Deserialize)]
pub struct JoinOrgaRequest {
    pub code: String,
}

#[derive(Serialize)]
pub struct AllEquivalentsResponse {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equivalents: Option<Vec<EquivalentSelection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub async fn update_account(
    session: Session,
    State(pool): State<MySqlPool>,
    AuthenticatedUser(mut user_full): AuthenticatedUser,
    Json(payload): Json<UpdateAccountRequest>,
) -> Result<Json<Value>, AppError> {
    let new_user = UserService::update_user(&pool, user_full.user, payload).await
        .map_err(AppError::AuthError)?;

    user_full.user = new_user;
    session.insert("user_full", user_full.clone()).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "account": user_full
    })))
}

pub async fn delete_account( session: Session, State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Result<Json<Value>, AppError> {
    let user_id = user_full.user.id;

    UserService::delete_user(&pool, user_id).await
        .map_err(|e| AppError::DatabaseError(e))?;

    session.remove::<UserFull>("user_full").await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "Votre compte a été supprimé avec succès"
    })))
}

pub async fn join_organization(
    session: Session,
    State(pool): State<MySqlPool>,
    AuthenticatedUser(mut user_full): AuthenticatedUser,
    Json(payload): Json<JoinOrgaRequest>,
) -> Result<Json<Value>, AppError> {
    let orga_code = payload.code.clone();
    let orga: Option<Organisation> = UserService::join_organization(&pool, orga_code, user_full.user.id).await
         .map_err(AppError::BadRequest)?;
    let organisation = orga.unwrap();

    user_full.user.id_organisation = Some(organisation.id);
    user_full.organisation = Some(organisation);
    session.insert("user_full", user_full).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "success.org_joined"
    })))
}

pub async fn get_organisation_member(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Result<Json<Value>, AppError> {
    let orga_id: i64 = user_full.organisation.ok_or(AppError::AuthError("errors.auth.unauthenticated".to_string()))?.id;

    let members = UserService::get_organization_members(&pool, orga_id).await.unwrap_or_else(|_| Vec::new());

    Ok(Json(json!({
        "success": true,
        "members": members
    })))
}

pub async fn remove_organisation_member(State(pool): State<MySqlPool>, Json(payload) : Json<Value>) -> Result<Json<Value>, AppError> {
    let user_id = payload["userId"].as_i64().ok_or(AppError::BadRequest("Missing userId".into()))?;

    UserService::remove_organization_member(&pool, user_id).await
        .map_err(AppError::from)?;

    Ok(Json(json!({
        "success": true
    })))
}

pub async fn update_organisation(session: Session, State(pool): State<MySqlPool>, AuthenticatedUser(mut user_full): AuthenticatedUser, Json(payload) : Json<UpdateOrganisationRequest>) -> Result<Json<Value>, AppError> {
    let org = user_full.organisation.as_ref().ok_or(AppError::AuthError("errors.auth.unauthenticated_org".to_string()))?;

    if !user_full.user.est_admin {
        return Err(AppError::AuthError("errors.auth.unauthenticated_org".to_string()));
    }

    let updated_org = OrganisationService::update_organisation(&pool, org, payload).await
        .map_err(AppError::BadRequest)?;

    user_full.organisation = Some(updated_org.clone());
    session.insert("user_full", user_full).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "organisation": updated_org
    })))
}

pub async fn leave_organization(
    session: Session,
    State(pool): State<MySqlPool>,
    AuthenticatedUser(mut user_full): AuthenticatedUser,
) -> Result<Json<Value>, AppError> {
    UserService::remove_organization_member(&pool, user_full.user.id).await
        .map_err(|e| AppError::InternalServerError(format!("Erreur quitter organisation: {}", e)))?;

    user_full.user.id_organisation = None;
    user_full.organisation = None;
    session.insert("user_full", user_full).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "success.org_left"
    })))
}

pub async fn get_my_organization(
    AuthenticatedUser(user_full): AuthenticatedUser
) -> Result<Json<Value>, AppError> {
    let orga = user_full.organisation.ok_or(AppError::NotFound("organisation not found".to_string()))?;

    Ok(Json(json!({
        "success": true,
        "organisation": orga
    })))
}

pub async fn get_account_all_equivalents(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Result<Json<AllEquivalentsResponse>, AppError> {
    let user_id = user_full.user.id;

    let equivalents = EquivalentService::get_all_equivalents_with_selection(&pool, user_id).await
        .map_err(AppError::from)?;

    Ok(Json(AllEquivalentsResponse {
        success: true,
        equivalents: Some(equivalents),
        message: None,
    }))
}

pub async fn update_account_equivalents(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser, Json(payload): Json<Value>) -> Result<Json<Value>, AppError> {
     let user_id = user_full.user.id;

    let equivalents: Vec<i64> = payload["equivalents"].as_array()
        .ok_or(AppError::BadRequest("Liste d'équivalents manquante ou invalide".to_string()))?
        .iter()
        .filter_map(|v| {
             if let Some(s) = v.as_str() {
                 s.parse::<i64>().ok()
             } else if let Some(i) = v.as_i64() {
                 Some(i)
             } else {
                 None
             }
        }).collect();

    EquivalentService::update_user_equivalents(&pool, user_id, equivalents).await
        .map_err(AppError::InternalServerError)?;

    Ok(Json(json!({ "success": true })))
}

pub async fn create_organization(
    State(pool): State<MySqlPool>,
    session: Session,
    AuthenticatedUser(mut user_full): AuthenticatedUser,
    Json(payload): Json<Value>
) -> Result<Json<CurrentAccountResponse>, AppError> {
    let orga_name = payload["organization_name"].as_str()
        .ok_or(AppError::BadRequest("Missing organization_name".to_string()))?;

    let siret: Option<String> = payload["siret"].as_str().map(|s| s.to_string());

    let user_id = user_full.user.id;

    if let Some(ref s) = siret {
        let exists = OrganisationService::find_id_by_siret(&pool, s.clone()).await
            .map_err(AppError::DatabaseError)?
            .is_some();

        if exists {
            return Err(AppError::BadRequest("errors.org_siret_exists".to_string()));
        }
    }

    let (organisation_id, organisation_code) = OrganisationService::inscription_orga(&pool, orga_name, siret.clone(), user_id).await
        .map_err(AppError::BadRequest)?;

    let organisation = Organisation {
        id: organisation_id,
        nom: orga_name.to_string(),
        siret,
        code: organisation_code,
    };

    user_full.organisation = Some(organisation);
    user_full.user.est_admin = true;

    session.insert("user_full", user_full.clone()).await.map_err(|_| AppError::InternalServerError("Session error".to_string()))?;

    Ok(Json(CurrentAccountResponse {
        success: true,
        user_full: Some(user_full),
        message: None,
    }))
}
