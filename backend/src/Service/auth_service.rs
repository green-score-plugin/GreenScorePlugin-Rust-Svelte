use sqlx::MySqlPool;
use crate::repository::user_repository::UserRepository;

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
        if UserRepository::find_id_by_email(pool, email).await.map_err(|_| "db_error")?.is_some() {
            return Err("email_exists".to_string());
        }

        let password_hash = Self::hash_password(password).map_err(|_| "hash_error")?;
        let user_id = UserRepository::insert_user(pool, email, &password_hash, first_name, last_name)
            .await.map_err(|_| "insert_error")?;

        Ok(user_id)
    }
}