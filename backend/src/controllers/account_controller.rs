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
) -> Json<Value> {
    let new_user = match UserService::update_user(&pool, user_full.user, payload).await{
        Ok(new_user) => new_user,
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("errors.auth.unauthenticated: {}", e),
        }))
    };

    user_full.user = new_user;
    session.insert("user_full", user_full.clone()).await.unwrap();

    Json(json!({
        "success": true,
        "account": user_full
    }))

}

pub async fn delete_account( session: Session, State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Json<Value> {
    let user_id = user_full.user.id;

    match UserService::delete_user(&pool, user_id).await {
        Ok(_) => {
            session.remove::<UserFull>("user_full").await.unwrap();
            Json(json!({
                "success": true,
                "message": "Votre compte a été supprimé avec succès"
            }))
        },
        Err(e) => Json(json!({
            "success": false,
            "message": format!("Erreur suppression compte: {}", e)
        })),
    }

}

pub async fn join_organization(
    session: Session,
    State(pool): State<MySqlPool>,
    AuthenticatedUser(mut user_full): AuthenticatedUser,
    Json(payload): Json<JoinOrgaRequest>,
) -> Json<Value> {
    let orga_code = payload.code.clone();
    let orga_id = UserService::join_organization(&pool, orga_code, user_full.user.id).await;

    match orga_id {
        Ok(id) => user_full.user.id_organisation = Some(id),
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("Erreur rejoindre organisation: {}", e)
        })),
    }

    session.insert("user_full", user_full).await.unwrap();

    Json(json!({
        "success": true,
        "message": "success.org_joined"
    }))
}

pub async fn get_organisation_member(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Json<Value> {
    let orga_id: i64 = match user_full.organisation {
        Some(org) => org.id,
        None => return Json(json!({
            "success": false,
            "message": "errors.auth.unauthenticated"
        }))
    };

    let members = UserService::get_organization_members(&pool, orga_id).await.unwrap_or_else(|_| Vec::new());

    Json(json!({
        "success": true,
        "members": members
    }))
}

pub async fn remove_organisation_member(State(pool): State<MySqlPool>, Json(payload) : Json<Value>) -> Json<Value> {

    let user_id = payload["userId"].as_i64().unwrap();

    UserService::remove_organization_member(&pool, user_id).await.unwrap_or_else(|_| ());

    Json(json!({
        "success": true
    }))

}

pub async fn update_organisation(session: Session, State(pool): State<MySqlPool>, AuthenticatedUser(mut user_full): AuthenticatedUser, Json(payload) : Json<UpdateOrganisationRequest>) -> Json<Value> {
    let org = match user_full.organisation.as_ref() {
        Some(o) => o,
        None => return Json(json!({ "success": false, "message": "errors.auth.unauthenticated_org" })),
    };

    if !user_full.user.est_admin {
        return Json(json!({ "success": false, "message": "errors.auth.unauthenticated_org" }));
    }

    match OrganisationService::update_organisation(&pool, org, payload).await {
        Ok(updated_org) => {
            user_full.organisation = Some(updated_org.clone());
            session.insert("user_full", user_full).await.unwrap();

            Json(json!({
                "success": true,
                "organisation": updated_org
            }))
        },
        Err(e) => Json(json!({ "success": false, "message": e })),
    }
}

pub async fn leave_organization(
    session: Session,
    State(pool): State<MySqlPool>,
    AuthenticatedUser(mut user_full): AuthenticatedUser,
) -> Json<Value> {
    match UserService::remove_organization_member(&pool, user_full.user.id).await {
        Ok(_) => (),
        Err(e) => return Json(json!({
            "success": false,
            "message": format!("Erreur quitter organisation: {}", e)
        })),
    }

    user_full.user.id_organisation = None;
    session.insert("user_full", user_full).await.unwrap();

    Json(json!({
        "success": true,
        "message": "success.org_left"
    }))
}

pub async fn get_my_organization(
    AuthenticatedUser(user_full): AuthenticatedUser
) -> Json<Value> {
    match user_full.organisation {
        Some(orga) => Json(json!({
            "success": true,
            "organisation": orga
        })),
        None => Json(json!({
        "success": false,
        "message": "organisation not found"}
        ))
    }
}

pub async fn get_account_all_equivalents(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser) -> Json<AllEquivalentsResponse> {
    let user_id = user_full.user.id;

    match EquivalentService::get_all_equivalents_with_selection(&pool, user_id).await {
        Ok(equivalents) => Json(AllEquivalentsResponse {
            success: true,
            equivalents: Some(equivalents),
            message: None,
        }),
        Err(e) => Json(AllEquivalentsResponse {
            success: false,
            equivalents: None,
            message: Some(format!("Erreur récupération équivalents: {}", e)),
        }),
    }
}

pub async fn update_account_equivalents(State(pool): State<MySqlPool>, AuthenticatedUser(user_full): AuthenticatedUser, Json(payload): Json<Value>) -> Json<Value> {
     let user_id = user_full.user.id;

    let equivalents: Vec<i64> = match payload["equivalents"].as_array() {
        Some(arr) => {
             arr.iter().filter_map(|v| {
                 if let Some(s) = v.as_str() {
                     s.parse::<i64>().ok()
                 } else if let Some(i) = v.as_i64() {
                     Some(i)
                 } else {
                     None
                 }
             }).collect()
        },
        None => return Json(json!({ "success": false, "message": "Liste d'équivalents manquante ou invalide" })),
    };

    match EquivalentService::update_user_equivalents(&pool, user_id, equivalents).await {
        Ok(_) => Json(json!({ "success": true })),
        Err(e) => Json(json!({ "success": false, "message": format!("Erreur mise à jour: {}", e) })),
    }
}
