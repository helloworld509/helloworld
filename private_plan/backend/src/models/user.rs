use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
