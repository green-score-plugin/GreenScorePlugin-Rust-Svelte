use sqlx::MySqlPool;
use crate::models::user::User;
use crate::models::organisation::Organisation;
use crate::models::service::Service;
use crate::dto::user_full::UserFull;
use crate::repository::user_repository::UserRepository;
use crate::repository::organisation_repository::OrganisationRepository;
use crate::repository::service_repository::ServiceRepository;
use crate::error::AppError;

pub struct AuthService;

impl AuthService{

    fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    pub async fn inscription(
        pool: &MySqlPool,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str)
        -> Result<i64, String>
    {
        if UserRepository::find_id_by_email(pool, email.parse().unwrap()).await.map_err(|_| "db_error")?.is_some() {
            return Err("email_exists".to_string());
        }

        let password_hash = Self::hash_password(password).map_err(|_| "hash_error")?;
        let user_id = UserRepository::insert_user(pool, email, &password_hash, first_name, last_name)
            .await.map_err(|_| "insert_error")?;

        Ok(user_id)
    }

    pub async fn login (pool: &MySqlPool, email: &str, password: &str) -> Result<UserFull, AppError> {
        let user_result = UserRepository::find_with_password_by_email(pool, email).await.map_err(AppError::DatabaseError)?;
        let (id, password_hash, first_name, last_name, service_id) = match user_result {
            Some(tuple) => tuple,
            None => return Err(AppError::AuthError("errors.auth.invalid_credentials".to_string())),
        };

        if !bcrypt::verify(password, &password_hash).map_err(|e| AppError::InternalServerError(e.to_string()))? {
            return Err(AppError::AuthError("errors.auth.invalid_credentials".to_string()));
        }

        let user = User {
            id,
            id_service: service_id,
            email: email.to_string(),
            prenom: first_name,
            nom: last_name,
            total_carbon_footprint: 0.0,
        };

        let mut service: Option<Service> = None;
        let mut organisations: Option<Vec<Organisation>> = Some(OrganisationRepository::find_all_by_user_id(pool, id).await.map_err(AppError::DatabaseError)?);

        if let Some(srv_id) = service_id {
            service = ServiceRepository::find_by_id(pool, srv_id).await.map_err(AppError::DatabaseError)?;
        }

        if let Some(ref srv) = service {
            let org_opt = OrganisationRepository::find_by_id(pool, srv.id_organisation).await.map_err(AppError::DatabaseError)?;
            organisations = org_opt.map(|o| vec![o]);
        }

        let user_full = UserFull {
            user,
            organisation: organisations,
            service,
        };

        Ok(user_full)
    }
}