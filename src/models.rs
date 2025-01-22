use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct PasswordVault {
    pub entries: Vec<PasswordEntry>,
}
