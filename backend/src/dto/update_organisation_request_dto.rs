use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateOrganisationRequest {
    pub id: Option<i64>,
    pub name: String,
    pub siret: Option<String>,
}
