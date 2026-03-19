use sqlx::MySqlPool;
use crate::repository::service_repository::ServiceRepository;

pub struct ServiceService;

impl ServiceService {
    pub async fn create_service(pool: &MySqlPool, organisation_id: i64, name: &str) -> Result<i64, String> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
             return Err("errors.validation_service_name_required".to_string());
        }

        if trimmed_name.is_empty() {
             return Err("name_required".to_string());
        }

        if ServiceRepository::find_by_nom(pool, trimmed_name, organisation_id).await.map_err(|e| format!("db_error: {}", e))?.is_some() {
            return Err("errors.service_exists".to_string());
        }

        ServiceRepository::create_service(pool, trimmed_name, organisation_id)
            .await
            .map_err(|e| format!("db_error: {}", e))
    }
}

