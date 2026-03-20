use sqlx::MySqlPool;
use crate::repository::service_repository::ServiceRepository;
use crate::models::service::Service;

pub struct ServiceService;

impl ServiceService {
    pub async fn create_service(pool: &MySqlPool, organisation_id: i64, name: &str) -> Result<Vec<Service>, String> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
             return Err("errors.validation_service_name_required".to_string());
        }

        if ServiceRepository::find_by_nom(pool, trimmed_name, organisation_id).await.map_err(|e| format!("db_error: {}", e))?.is_some() {
            return Err("errors.service_exists".to_string());
        }

        ServiceRepository::create_service(pool, trimmed_name, organisation_id)
            .await
            .map_err(|e| format!("db_error: {}", e))?;

        let services = ServiceRepository::get_services_by_organisation_id(pool, organisation_id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(services)
    }

    pub async fn get_organisation_services(pool: &MySqlPool, organisation_id: i64) -> Result<Vec<Service>, String> {
        ServiceRepository::get_services_by_organisation_id(pool, organisation_id)
            .await
            .map_err(|e| format!("db_error: {}", e))
    }

    pub async fn get_services_by_user_id(pool: &MySqlPool, user_id: i64) -> Result<Vec<Service>, String> {
        ServiceRepository::find_by_user_id(pool, user_id)
            .await
            .map_err(|e| format!("db_error: {}", e))
    }
}
