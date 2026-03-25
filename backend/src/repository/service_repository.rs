use sqlx::MySqlPool;
use crate::models::service::Service;

pub struct ServiceRepository;

impl ServiceRepository {

    pub async fn find_by_id(pool: &MySqlPool, service_id: i64) -> Result<Option<Service>, sqlx::Error>
    {
        let service = sqlx::query_as::<_, Service>(
            "SELECT id, nom, id_organisation FROM service WHERE id = ?"
        )
        .bind(service_id)
        .fetch_optional(pool)
        .await?;

        Ok(service)
    }

}