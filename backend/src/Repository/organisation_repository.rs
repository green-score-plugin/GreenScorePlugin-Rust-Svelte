use crate::models::organisation::Organisation;
pub struct OrganisationRepository;

impl OrganisationRepository {

    pub async fn find_by_id(pool: &sqlx::MySqlPool, organisation_id: i64) -> Result<Option<Organisation>, sqlx::Error> {
        let organisation = sqlx::query_as!(
            Organisation,
            "SELECT id, organisation_name, organisation_code, siret FROM organisations WHERE id = ?",
            organisation_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(organisation)
    }

    pub async fn find_id_by_siret(pool: &sqlx::MySqlPool, siret: &str) -> Result<Option<i64>, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT id FROM organisations WHERE siret = ?",
            siret
        )
        .fetch_optional(pool)
        .await?;

        Ok(result.map(|record| record.id))
    }

    pub async fn insert_organisation(pool: &sqlx::MySqlPool,
                                 organisation_name: &str,
                                 organisation_code: &str,
                                 siret: Option<&str>) -> Result<i64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO organisations (organisation_name, organisation_code, siret) VALUES (?, ?, ?)"
        )
        .execute(pool)
        .bind(organisation_name)
        .bind(organisation_code)
        .bind(siret)
        .await?;
        Ok(result.last_insert_id() as i64).expect("Failed to retrieve last insert ID");
    }

}