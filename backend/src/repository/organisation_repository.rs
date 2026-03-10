use crate::models::organisation::Organisation;
use sqlx::Row;

pub struct OrganisationRepository;

impl OrganisationRepository {

    pub async fn find_by_id(pool: &sqlx::MySqlPool, organisation_id: i64) -> Result<Option<Organisation>, sqlx::Error> {
        let organisation = sqlx::query_as::<_, Organisation>(
            "SELECT id, organisation_name, organisation_code, siret FROM organisation WHERE id = ?"
        )
        .bind(organisation_id)
        .fetch_optional(pool)
        .await?;

        Ok(organisation)
    }

    pub async fn find_id_by_siret(pool: &sqlx::MySqlPool, siret: &str) -> Result<Option<i64>, sqlx::Error> {
        let result = sqlx::query(
            "SELECT id FROM organisation WHERE siret = ?"
        )
        .bind(siret)
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|row| row.get::<i64, _>("id")))
    }

    pub async fn insert_organisation(pool: &sqlx::MySqlPool,
                                 organisation_name: &str,
                                 organisation_code: &str,
                                 siret: Option<&str>) -> Result<i64, sqlx::Error> {
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

}