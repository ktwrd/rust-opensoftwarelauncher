use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize)]
pub struct Connected {
    pub Uptime: i32,
    pub Version: String,
    pub AuthProvider: String,
    pub AuthProviderSignup: String
}


#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    pub Success: bool,
    pub Data: _data,
     
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DataType: Option<String>
}


#[derive(Debug, Clone, Deserialize)]
pub struct _data {
    pub Success: bool,
    pub Message: String,
       
    pub Groups: Vec<String>,
    pub Permissions: Vec<i32>,
    pub Token: _token
}

#[derive(Debug, Clone, Deserialize)]
pub struct _token {
    Allow: bool,
    Token: String,
    TokenHash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    useragent: Option<String>,
    Host: String,
    CreatedTimestamp: u128,
    LastUsed: u128
}

