use serde::Deserialize;
#[derive(Deserialize, Default)]
pub struct UpdateAccountRequest {
    #[serde(default)]
    pub email: Option<String>,

    #[serde(default)]
    pub prenom: Option<String>,

    #[serde(default)]
    pub nom: Option<String>,

    #[serde(default)]
    pub password: Option<String>,
}