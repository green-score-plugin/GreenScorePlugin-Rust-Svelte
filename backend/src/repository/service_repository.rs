use sqlx::MySqlPool;
use crate::models::service::Service;

pub struct ServiceRepository;

impl ServiceRepository {

    pub async fn find_by_id(pool: &MySqlPool, service_id: i64) -> Result<Option<Service>, sqlx::Error>
    {
        let service = sqlx::query_as::<_, Service>(
            "SELECT id, nom, organisation_id FROM service WHERE id = ?"
        )
        .bind(service_id)
        .fetch_optional(pool)
        .await?;

        Ok(service)
    }

    pub async fn create_service(pool: &MySqlPool, service_name: &str, organisation_id: i64) -> Result<i64, sqlx::Error>
    {
        let result = sqlx::query(
            "INSERT INTO service (nom, organisation_id) VALUES (?, ?)"
        )
        .bind(service_name)
        .bind(organisation_id)
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

}