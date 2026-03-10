use sqlx::MySqlPool;
use crate::models::service::Service;

pub struct ServiceRepository;

impl ServiceRepository {

    pub async fn find_by_id(pool: &MySqlPool, service_id: i64) -> Result<Option<Service>, sqlx::Error>
    {
        let service = sqlx::query_as!(
            Service,
            "SELECT id, service_name, organisation_id FROM services WHERE id = ?"
        )
        .fetch_optional(pool)
        .bind(service_id)
        .await?;

        Ok(service)
    }

}