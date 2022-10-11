use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Details {
    pub url: String,
    pub username: String,
    pub password: String,
    pub token: String
}
