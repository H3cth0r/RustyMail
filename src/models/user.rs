use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password_hash: String,
    pub aliases: Vec<String>,
}