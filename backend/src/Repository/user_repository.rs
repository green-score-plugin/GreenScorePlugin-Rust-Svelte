pub struct UserRepository;

impl UserRepository {
    pub async fn insert_user(pool: &sqlx::MySqlPool,
                             email: &str,
                             password_hash: &str,
                             first_name: &str,
                             last_name: &str) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO users (email, password_hash, first_name, last_name) VALUES (?, ?, ?, ?)"
        )
        .execute(pool)
        .bind(email)
        .bind(password_hash)
        .bind(first_name)
        .bind(last_name)
        .await?;
        Ok(result.last_insert_id() as i64).expect("Failed to retrieve last insert ID");
    }

    pub async fn find_id_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<i64>, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT id FROM users WHERE email = ?",
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|record| record.id))
    }
}