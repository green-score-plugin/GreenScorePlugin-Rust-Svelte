use sqlx::Row;

pub struct UserRepository;

impl UserRepository {
    pub async fn insert_user(pool: &sqlx::MySqlPool,
                             email: &str,
                             password_hash: &str,
                             first_name: &str,
                             last_name: &str) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user (email, password, first_name, last_name) VALUES (?, ?, ?, ?)"
        )
        .bind(email)
        .bind(password_hash)
        .bind(first_name)
        .bind(last_name)
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    pub async fn find_id_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<i64>, sqlx::Error> {
        let result = sqlx::query(
            "SELECT id FROM user WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| row.get::<i64, _>("id")))
    }

    pub async fn find_with_password_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<(i64, String, String, String, Option<i64>, Option<i64>, bool)>, sqlx::Error> {
        let result = sqlx::query(
            "SELECT id, password, first_name, last_name, organisation_id, service_id ,est_admin FROM user WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| (
            row.get::<i64, _>("id"),
            row.get::<String, _>("password"),
            row.get::<String, _>("first_name"),
            row.get::<String, _>("last_name"),
            row.get::<Option<i64>, _>("organisation_id"),
            row.get::<Option<i64>, _>("service_id"),
            row.get::<bool, _>("est_admin"),
        )))
    }

    pub async fn join_organisation(pool: &sqlx::MySqlPool, user_id: i64, organisation_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE user SET organisation_id = ? AND est_admin = true WHERE id = ?"
        )
        .bind(organisation_id)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}