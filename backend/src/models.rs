use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub nom: String,
    pub prenom: String,
}

impl User {
    pub fn new(id: i64, email: String, nom: String, prenom: String) -> Self {
        User { id, email, nom, prenom }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organisation {
    pub id: i64,
    pub nom: String,
    pub siret: Option<String>,
    pub code: String,
}

impl Organisation {
    pub fn new(id: i64, nom: String, siret: Option<String>, code: String) -> Self {
        Organisation { id, nom, siret, code }
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
}