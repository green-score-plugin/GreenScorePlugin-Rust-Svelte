pub struct UserRepository;

impl UserRepository {
    pub async fn insert_user(pool: &sqlx::MySqlPool,
                             email: &str,
                             password_hash: &str,
                             first_name: &str,
                             last_name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (email, password_hash, first_name, last_name) VALUES (?, ?, ?, ?)"
        )
        .execute(pool)
        .bind(email)
        .bind(password_hash)
        .bind(first_name)
        .bind(last_name)
        .await?;
        Ok(())
    }
}