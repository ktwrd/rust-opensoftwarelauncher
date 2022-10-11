use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize)]
pub struct Details {
    pub url: String,
    pub username: String,
    pub password: String
}
