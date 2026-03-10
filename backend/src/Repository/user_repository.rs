use crate::models::user::User;

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

    pub async fn find_with_password_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<(i64, String, String, String, Option<i64>, Option<i64>, bool)>, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT id, password_hash, first_name, last_name, organisation_id, service_id ,est_admin FROM users WHERE email = ?"
        )
        .fetch_optional(pool)
        .bind(email)
        .await?;

        Ok(result.map(|record| (record.id, record.password_hash, record.first_name, record.last_name, record.organisation_id, record.service_id, record.est_admin)))
    }

    pub async fn join_organisation(pool: &sqlx::MySqlPool, user_id: i64, organisation_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE users SET organisation_id = ? AND est_admin = true WHERE id = ?"
        )
        .execute(pool)
        .bind(organisation_id)
        .bind(user_id)
        .await?;
        Ok(())
    }
}