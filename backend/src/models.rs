use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub prenom: String,
    pub nom: String,
    #[serde(default)]
    pub id_orga: Option<i64>,
}

impl User {
    pub fn new(id: i64, email: String, nom: String, prenom: String, id_orga: Option<i64>) -> Self {
        User { id, email, nom, prenom, id_orga }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organisation {
    pub id: i64,
    pub nom: String,
    pub siret: Option<String>,
    pub code: String,
    pub admin_id: i64
}

impl Organisation {
    pub fn new(id: i64, nom: String, siret: Option<String>, code: String, admin_id: i64) -> Self {
        Organisation { id, nom, siret, code, admin_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum Account {
    User(User),
    Organisation(Organisation),
}
impl Account {
    pub fn id(&self) -> i64 {
        match self {
            Account::User(u) => u.id,
            Account::Organisation(o) => o.id,
        }
    }

    pub fn organization_id(&self) -> Option<i64> {
        match self {
            Account::User(u) => u.id_orga,
            Account::Organisation(o) => Some(o.id),
        }
    }
}