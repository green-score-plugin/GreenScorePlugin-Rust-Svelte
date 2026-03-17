use sqlx::{MySqlPool, Row, Error};
use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn insert_user(
        pool: &MySqlPool,
        email: &str,
        password_hash: &str,
        first_name: &str,
        last_name: &str
    ) -> Result<i64, Error> {
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

    pub async fn find_id_by_email(
        pool: &MySqlPool,
        email: String
    ) -> Result<Option<i64>, Error> {
        let result = sqlx::query(
            "SELECT id FROM user WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| row.get::<i64, _>("id")))
    }

    pub async fn find_with_password_by_email(
        pool: &MySqlPool,
        email: &str
    ) -> Result<Option<(i64, String, String, String, Option<i64>, Option<i64>, bool)>, Error> {
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

    pub async fn join_organisation(
        pool: &MySqlPool,
        user_id: i64,
        organisation_id: i64,
        is_admin: bool
    ) -> Result<(), Error> {
        sqlx::query(
            "UPDATE user SET organisation_id = ?, est_admin ? true WHERE id = ?"
        )
        .bind(organisation_id)
        .bind(is_admin)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_total_carbon_footprint_by_id(
        pool: &MySqlPool,
        user_id: i64,
        carbon_footprint: f64
    ) -> Result<(), Error> {
        sqlx::query(
            "UPDATE user SET total_carbon_footprint = COALESCE(total_carbon_footprint, 0) + ? WHERE id = ?"
        )
        .bind(carbon_footprint)
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn find_total_carbon_footprint_by_id(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<Option<f64>, Error> {
        let result = sqlx::query(
            "SELECT total_carbon_footprint FROM user WHERE id = ?"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| row.get::<Option<f64>, _>("total_carbon_footprint").unwrap_or(0.0)))
    }

    pub async fn count_user_equivalent(
        pool: &MySqlPool,
        user_id: i64
    ) -> Result<i64, Error> {
        let result = sqlx::query(
            "SELECT COUNT(*) FROM user_equivalent WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        Ok(result.get::<i64, _>(0))
    }

    pub async fn update_user(
        pool: &MySqlPool,
        new_user: User,
        new_password: Option<String>
    ) -> Result<(), Error> {
        let mut query = String::from("UPDATE user SET ");
        let mut updates: Vec<String> = Vec::new();
        let mut params: Vec<String> = Vec::new();

        updates.push("email = ?".to_string());
        params.push(new_user.email.clone());

        updates.push("first_name = ?".to_string());
        params.push(new_user.prenom.clone());

        updates.push("last_name = ?".to_string());
        params.push(new_user.nom.clone());

        if new_password.is_some() {
            updates.push("password = ?".to_string());
            params.push(new_password.unwrap());
        }

        query.push_str(&updates.join(", "));
        query.push_str(" WHERE id = ?");

        let mut q = sqlx::query(&query);
        for param in params {
            q = q.bind(param);
        }
        q = q.bind(new_user.id);

        q.execute(pool).await?;
        Ok(())
    }

    pub async fn delete_user(pool: &MySqlPool, user_id: i64) -> Result<(), Error> {
        sqlx::query(
            "DELETE FROM user WHERE id = ?"
        )
        .bind(user_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update_user_organization(pool: &MySqlPool, user_id: i64, orga_id: i64) -> Result<(), Error> {
        sqlx::query(
            "UPDATE user SET organisation_id = ? WHERE id = ?"
        )
        .bind(orga_id)
        .bind(user_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_organization_members(pool: &MySqlPool, orga_id: i64) -> Result<Vec<User>, Error> {
        let result = sqlx::query_as::<_, User>(
            "SELECT * FROM user WHERE organisation_id = ? AND est_admin = true"
        )
            .bind(orga_id)
            .fetch_all(pool)
            .await?;
        Ok(result)
    }

    pub async fn remove_organization_member(pool: &MySqlPool, user_id: i64) -> Result<(), Error> {
        sqlx::query("
            UPDATE user SET organisation_id = NULL WHERE id = ?
        ")
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}