use serde::{Deserialize};


#[derive(Debug, Clone, Deserialize)]
pub struct Connected {
    #[serde(rename = "Uptime")]
    pub uptime: i32,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "AuthProvider")]
    pub authprovider: String,
    #[serde(rename = "AuthProviderSignup")]
    pub authprovidersignup: String
}


#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "Data")]
    pub data: TokenData,
     
    #[serde(skip_serializing_if = "Option::is_none", rename = "DataType" )]
    pub datatype: Option<String>
}


#[derive(Debug, Clone, Deserialize)]
pub struct TokenData {
    #[serde(rename = "Success")]
    pub success: bool,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Group")]   
    pub groups: Vec<String>,
    #[serde(rename = "Permissions")] 
    pub permissions: Vec<i32>,
    #[serde(rename = "Token")] 
    pub token: DataToken
}

#[derive(Debug, Clone, Deserialize)]
pub struct DataToken {
    #[serde(rename = "Allow")] 
    pub allow: bool,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "TokenHash")]
    pub tokenhash: String,

    /* there is no current use for these
    #[serde(skip_serializing_if = "Option::is_none")]
    useragent: Option<String>,
    Host: String,
    CreatedTimestamp: u128,
    LastUsed: u128
    */
}

