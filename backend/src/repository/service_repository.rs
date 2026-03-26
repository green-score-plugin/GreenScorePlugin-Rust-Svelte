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
    
    pub async fn find_by_nom(pool: &MySqlPool, nom: &str, organisation_id: i64) -> Result<Option<Service>, sqlx::Error>
    {
        let service = sqlx::query_as::<_, Service>(
            "SELECT id, nom, id_organisation FROM service WHERE nom = ? AND id_organisation = ?"
        )
        .bind(nom)
        .bind(organisation_id)
        .fetch_optional(pool)
        .await?;

        Ok(service)
    }

    pub async fn create_service(pool: &MySqlPool, service_name: &str, organisation_id: i64) -> Result<i64, sqlx::Error>
    {
        let result = sqlx::query(
            "INSERT INTO service (nom, id_organisation) VALUES (?, ?)"
        )
        .bind(service_name)
        .bind(organisation_id)
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as i64)
    }

    pub async fn get_services_by_organisation_id(pool: &MySqlPool, organisation_id: i64) -> Result<Vec<Service>, sqlx::Error> {
        let services = sqlx::query_as::<_, Service>(
            "SELECT id, nom, id_organisation FROM service WHERE id_organisation = ?"
        )
        .bind(organisation_id)
        .fetch_all(pool)
        .await?;

        Ok(services)
    }

    pub async fn find_by_user_id(pool: &MySqlPool, user_id: i64) -> Result<Vec<Service>, sqlx::Error> {
        let services = sqlx::query_as::<_, Service>(
            r#"
            SELECT s.id, s.nom, s.id_organisation
            FROM service s
            JOIN user u ON u.id_organisation = s.id_organisation
            WHERE u.id = ?
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await?;

        Ok(services)
    }

    pub async fn delete_by_id(pool: &MySqlPool, service_id: i64) -> Result<(), sqlx::Error> {
        // Disassociate users from this service first
        sqlx::query("UPDATE user SET service_id = NULL WHERE service_id = ?")
            .bind(service_id)
            .execute(pool)
            .await?;

        // Then delete the service
        sqlx::query("DELETE FROM service WHERE id = ?")
            .bind(service_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}