use crate::models::organisation::Organisation;
use sqlx::Row;
use sqlx::MySqlPool;

pub struct OrganisationRepository;

impl OrganisationRepository {

    pub async fn find_by_id(pool: &MySqlPool, organisation_id: i64) -> Result<Option<Organisation>, sqlx::Error> {
        let organisation = sqlx::query_as::<_, Organisation>(
            "SELECT id, organisation_name, organisation_code, siret FROM organisation WHERE id = ?"
        )
        .bind(organisation_id)
        .fetch_optional(pool)
        .await?;

        Ok(organisation)
    }

    pub async fn find_id_by_siret(pool: &MySqlPool, siret: &str) -> Result<Option<i64>, sqlx::Error> {
        let result = sqlx::query(
            "SELECT id FROM organisation WHERE siret = ?"
        )
        .bind(siret)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| row.get::<i64, _>("id")))
    }

    pub async fn insert_organisation(pool: &MySqlPool,
                                 organisation_name: &str,
                                 organisation_code: &str,
                                 siret: Option<String>) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO organisation (organisation_name, organisation_code, siret) VALUES (?, ?, ?)"
        )
        .bind(organisation_name)
        .bind(organisation_code)
        .bind(siret)
        .execute(pool)
        .await?;
        Ok(result.last_insert_id() as i64)
    }

    pub async fn organization_name(pool: &MySqlPool, orga_id: i64) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, String>(
            "SELECT organisation_name FROM organisation WHERE id = ?"
        )
            .bind(orga_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_id_by_name(pool: &MySqlPool, name: &str) -> Result<Option<i64>, sqlx::Error> {
        sqlx::query_scalar("SELECT id FROM organisation WHERE organisation_name = ?")
            .bind(name)
            .fetch_optional(pool)
            .await
    }

    pub async fn update_organisation(
        pool: &MySqlPool,
        id: i64,
        name: &str,
        siret: Option<String>
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE organisation SET organisation_name = ?, siret = ? WHERE id = ?")
            .bind(name)
            .bind(siret)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn find_organization_by_code(pool: &MySqlPool, org_code: String) -> Result<Option<Organisation>, sqlx::Error> {
        sqlx::query_as::<_, Organisation>(
            "SELECT * FROM organisation WHERE organisation_code = ?"
        )
            .bind(org_code)
            .fetch_optional(pool)
            .await
    }
}