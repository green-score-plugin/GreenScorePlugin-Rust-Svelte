use sqlx::{Error, MySqlPool};
use crate::models::user::User;
use crate::repository::user_repository::UserRepository;
use crate::dto::update_account_request_dto::UpdateAccountRequest;
use crate::repository::organisation_repository::OrganisationRepository;

pub struct UserService;

impl UserService {
    pub async fn user_with_email_exists(
        pool: &MySqlPool,
        email: String
    ) -> bool {
        let result = UserRepository::find_id_by_email(&pool, email).await.unwrap_or(None);
        result.is_some()
    }

    pub async fn update_user(
        pool: &MySqlPool,
        user: User,
        payload: UpdateAccountRequest,
    ) -> Result<User, String> {

        let mut new_user = user.clone();

        if let Some(email) = payload.email.as_ref().filter(|e| *e != &new_user.email) {
            let user_exists: bool = UserService::user_with_email_exists(&pool, email.to_string()).await;
            if user_exists {
                return Err("Cet email est déjà utilisé".to_string());
            }
            new_user.email = email.to_string();
        }

        if let Some(ref prenom) = payload.prenom {
            new_user.prenom = prenom.clone();
        }

        if let Some(ref nom) = payload.nom {
            new_user.nom = nom.clone();
        }

        let mut hash: Option<String> = None;
        if let Some(ref password) = payload.password {
            hash = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
                Ok(h) => Some(h),
                Err(_) => {
                    return Err("errors.auth.hash_error".to_string());
                }
            };
        }

        match UserRepository::update_user(pool, new_user.clone(), hash).await {
            Ok(_) => Ok(new_user),
            Err(e) => Err(format!("Erreur mise à jour: {}", e)),
        }
    }

    pub async fn delete_user(pool: &MySqlPool, user_id: i64) -> Result<(), Error> {
        UserRepository::delete_user(&pool, user_id).await
    }

    pub async fn join_organization(pool: &MySqlPool, orga_code: String, user_id: i64) -> Result<i64, String> {
        let orga = OrganisationRepository::find_organization_by_code(pool, orga_code)
            .await
            .map_err(|e| e.to_string())?;

        let orga_id = orga.ok_or("errors.org_code_invalid".to_string())?.id;

        UserRepository::update_user_organization(pool, user_id, orga_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(orga_id)

    }

    pub async fn get_organization_members(pool: &MySqlPool, orga_id: i64) -> Result<Vec<User>, Error> {
        UserRepository::get_organization_members(pool, orga_id).await
    }

    pub async fn remove_organization_member(pool: &MySqlPool, user_id: i64) -> Result<(), Error> {
        UserRepository::remove_organization_member(pool, user_id).await
    }
}