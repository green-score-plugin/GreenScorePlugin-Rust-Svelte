use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateOrganisationRequest {
    pub name: String,
    pub siret: Option<String>,
}

