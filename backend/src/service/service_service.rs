use sqlx::MySqlPool;
use crate::repository::service_repository::ServiceRepository;

pub struct ServiceService;

impl ServiceService {
    pub async fn create_service(pool: &MySqlPool, organisation_id: i64, name: &str) -> Result<i64, String> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
             return Err("name_required".to_string());
        }

        ServiceRepository::create_service(pool, trimmed_name, organisation_id)
            .await
            .map_err(|_| "db_error".to_string())
    }
}

