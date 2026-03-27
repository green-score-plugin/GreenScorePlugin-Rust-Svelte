use sqlx::MySqlPool;
use crate::repository::service_repository::ServiceRepository;
use crate::repository::user_repository::UserRepository;
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

    pub async fn delete_service(pool: &MySqlPool, service_id: i64, organisation_id: i64) -> Result<(), String> {
        let service = ServiceRepository::find_by_id(pool, service_id).await.map_err(|e| format!("db_error: {}", e))?;
        if let Some(s) = service {
            if s.id_organisation != organisation_id {
                 return Err("errors.service_not_in_organisation".to_string());
            }
            ServiceRepository::delete_by_id(pool, service_id)
                .await
                .map_err(|e| format!("db_error: {}", e))?;
            Ok(())
        } else {
             Err("errors.service_not_found".to_string())
        }
    }

    pub async fn assign_user_to_service(pool: &MySqlPool, user_id: i64, service_id: i64, organisation_id: i64) -> Result<(), String> {
        // Verify service belongs to organization
        let service = ServiceRepository::find_by_id(pool, service_id).await.map_err(|e| format!("db_error: {}", e))?;
        match service {
            Some(s) => {
                if s.id_organisation != organisation_id {
                    return Err("errors.service_not_in_organisation".to_string());
                }
            },
            None => return Err("errors.service_not_found".to_string()),
        }

        let is_member = UserRepository::is_member_of_organisation(pool, user_id, organisation_id)
            .await
            .map_err(|e| format!("db_error: {}", e))?;

        if !is_member {
             return Err("errors.user_not_in_organisation".to_string());
        }

        UserRepository::update_user_service(pool, user_id, Some(service_id))
            .await
            .map_err(|e| format!("db_error: {}", e))?;

        Ok(())
    }

    pub async fn unassign_user_from_service(pool: &MySqlPool, user_id: i64, organisation_id: i64) -> Result<(), String> {
        let is_member = UserRepository::is_member_of_organisation(pool, user_id, organisation_id)
            .await
            .map_err(|e| format!("db_error: {}", e))?;

        if !is_member {
             return Err("errors.user_not_in_organisation".to_string());
        }

        UserRepository::update_user_service(pool, user_id, None)
            .await
            .map_err(|e| format!("db_error: {}", e))?;

        Ok(())
    }
}
