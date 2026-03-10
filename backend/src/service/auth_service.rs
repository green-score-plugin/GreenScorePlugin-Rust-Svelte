use rand::Rng;
use sqlx::MySqlPool;
use crate::models::user::User;
use crate::models::organisation::Organisation;
use crate::models::service::Service;
use crate::dto::user_full::UserFull;
use crate::repository::user_repository::UserRepository;
use crate::repository::organisation_repository::OrganisationRepository;
use crate::repository::service_repository::ServiceRepository;

pub struct AuthService;

impl AuthService{

    fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        bcrypt::hash(password, bcrypt::DEFAULT_COST)
    }

    fn generate_organisation_code() -> String {
        const CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        const LENGTH: usize = 8;

        let mut rng = rand::rng();

        (0..LENGTH)
            .map(|_| {
                let idx = rng.random_range(0..CHARACTERS.len());
                CHARACTERS[idx] as char
            })
            .collect()
    }

    pub async fn inscription(
        pool: &MySqlPool,
        email: &str,
        password: &str,
        first_name: &str,
        last_name: &str)
        -> Result<i64, String>
    {
        if UserRepository::find_id_by_email(pool, email).await.map_err(|_| "db_error")?.is_some() {
            return Err("email_exists".to_string());
        }

        let password_hash = Self::hash_password(password).map_err(|_| "hash_error")?;
        let user_id = UserRepository::insert_user(pool, email, &password_hash, first_name, last_name)
            .await.map_err(|_| "insert_error")?;

        Ok(user_id)
    }

    pub async fn inscription_orga(
        pool: &MySqlPool,
        organisation_name: &str,
        siret: Option<&str>,
        user_id: i64
    ) -> Result<(i64, String), String>
    {
        if OrganisationRepository::find_id_by_siret(pool, organisation_name).await.map_err(|_| "db_error")?.is_some() {
            return Err("organisation_exists".to_string());
        }

        let code = Self::generate_organisation_code();

        let organisation_id = OrganisationRepository::insert_organisation(pool, organisation_name, &code, siret)
            .await
            .map_err(|_| "insert_error")?;

        UserRepository::join_organisation(pool, user_id, organisation_id)
            .await
            .map_err(|_| "join_error")?;

        Ok((organisation_id, code))
    }

    pub async fn login (pool: &MySqlPool, email: &str, password: &str) -> Result<UserFull, String> {
        let user_result = UserRepository::find_with_password_by_email(pool, email).await.map_err(|_| "db_error")?;
        let (id, password_hash, first_name, last_name, organisation_id, service_id, est_admin) = match user_result {
            Some(tuple) => tuple,
            None => return Err("user_not_found".to_string()),
        };

        if !bcrypt::verify(password, &password_hash).map_err(|_| "hash_error")? {
            return Err("invalid_credentials".to_string());
        }

        let user = User {
            id,
            id_organisation: organisation_id,
            id_service: service_id,
            email: email.to_string(),
            prenom: first_name,
            nom: last_name,
            est_admin,
            total_carbon_footprint: 0.0,
        };

        let mut organisation: Option<Organisation> = None;
        let mut service: Option<Service> = None;

        if let Some(org_id) = organisation_id {
            organisation = OrganisationRepository::find_by_id(pool, org_id).await.map_err(|_| "db_error")?;
        }

        if let Some(srv_id) = service_id {
            service = ServiceRepository::find_by_id(pool, srv_id).await.map_err(|_| "db_error")?;
        }

        if let Some(ref srv) = service {
            organisation = OrganisationRepository::find_by_id(pool, srv.id_organisation).await.map_err(|_| "db_error")?;
        }

        let user_full = UserFull {
            user,
            organisation,
            service,
        };

        Ok(user_full)
    }
}