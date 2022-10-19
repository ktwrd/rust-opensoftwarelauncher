use serde::{Deserialize, Serialize};


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



/*
#[derive(Debug, Clone, Deserialize)]
pub struct BuildVec {
    Vec<Build>
}
*/

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Build {
    #[serde(rename = "UID")]
    pub uid: String,
    #[serde(rename = "ProductName")]
    pub productname: String,
    #[serde(rename = "ProductID")]
    pub productid: String,
    #[serde(rename = "Streams")]
    pub streams: Vec<BuildStream>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildStream {

    #[serde(rename = "UID")]
    pub uid: String,
    #[serde(rename = "ProductID")]
    pub productid: String,
    #[serde(rename = "ProductName")]
    pub productname: String,
    #[serde(rename = "ProductVersion")]
    pub productversion: String,
    #[serde(rename = "ProductExpiryTimestamp")]
    pub productexpirytimestamp: u32,
    #[serde(rename = "ProductExpiryAt")]
    pub productexpiryat: String,
     #[serde(rename = "BranchName")]
    pub branchname: String,
    #[serde(rename = "UpdatedTimestamp")]
    pub updatedtimestamp: u64,
    #[serde(rename = "UpdatedAt")]
    pub updatedat: String,
    #[serde(rename = "RemoteSignature")]
    pub remotesignature: String,
    #[serde(rename = "Executable")]
    pub executable: BuildExecutables,
    #[serde(rename = "CommitHash")]
    pub commithash: String,
    #[serde(rename = "GroupWhitelist")]
    pub groupwhitelist: Vec<String>,
    #[serde(rename = "GroupBlacklist")]
    pub groupblacklist: Vec<String>,

}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildExecutables {
    #[serde(rename = "UID")]
    pub uid: String,
    #[serde(rename = "Linux")]
    pub linux: String,
    #[serde(rename = "Windows")]
    pub windows: String
}















