use rand::Rng;
use sqlx::MySqlPool;
use crate::repository::user_repository::UserRepository;
use crate::repository::organisation_repository::OrganisationRepository;

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
            .await.map_err(|_| "insert_error")?;

        UserRepository::join_organisation(pool, user_id, organisation_id)
            .await.map_err(|_| "join_error")?;


        Ok((organisation_id, code))
    }
}