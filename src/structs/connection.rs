use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize)]
pub struct Connected {
    pub Uptime: i32,
    pub Version: String,
    pub AuthProvider: String,
    pub AuthProviderSignup: String
}
