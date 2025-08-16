use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}
