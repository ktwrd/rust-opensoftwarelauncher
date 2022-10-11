use serde_json::{Result};
use tokio;

mod com;
use com::*;

mod fs;
use fs::*;

pub mod structs {
    pub mod connection;
    pub mod details;
}
// internal crates
use crate::structs::details::Details;
use crate::structs::connection::TokenResponse;


#[tokio::main]
async fn main() {

    let d: Details = details_deser();

    let connection = osl_connect_deser(osl_connect(d.url.clone()).await);

    match d.token.clone().len() {
        0 => {
        let token = token_response_deser(osl_token_grant(d.clone()).await).Data.Token.Token;
        println!("this program requires a token, please inser in details.json\n{token}");
        return
        },
        _ => d.clone().token
    };
    

    let mut args = std::env::args();

    if args.len() < 3 {

         let connect = osl_connect_deser(osl_connect(d.url.clone()).await);
        println!("Connected Successfully\nUptime: {}\nVersion: {}\n", connect.Uptime, connect.Version);
        return

    };


    let a1 = args.nth(1).unwrap();
    let a2 = args.nth(0).unwrap();

    match &a1 as &str {

        "--version" => { 

            match &a2 as &str {
                
                "current" => println!("{}", osl_connect_deser(osl_connect(d.url.clone()).await).Version),
                            
                "latest" => println!("{:?}", osl_release(d.clone(), String::from("latest")).await),
                             
                _ => println!("Options\n--current, --latest")
            };


        },

        _ => println!("invalid Command"),
    };


}
                   

    /*

    Uptime: 15886
Version: 1.0.76.513
http://buildservice.api.minalogger.com/token/grant?username=toastxc%40proton.me&password=mhSayrWym7pEiPafFtktX
Details { url: "http://buildservice.api.minalogger.com", username: "toastxc%40proton.me", password: "mhSayrWym7pEiPafFtktX", token: "" }

Connected { Uptime: 15886, Version: "1.0.76.513", AuthProvider: "https://minalogger.com", AuthProviderSignup: "https://minalogger.com/register" }

TokenResponse { Success: true, Data: _data { Success: true, Message: "ServerResponse_Account_TokenGranted", Groups: [], Permissions: [], Token: _token { Allow: true, Token: "67P521Q267CYNQL2M1RR5IGHYKNB7TIE", TokenHash: "b2b290f5c795829ee95b691fd259df3ffee1dc9b00e5114c26ff3145bea29a62", useragent: None, Host: "125.253.35.22", CreatedTimestamp: 1665475999002, LastUsed: 1665475999029 } }, DataType: Some("OSLCommon.Authorization.GrantTokenResponse, OSLCommon, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null") }
*/

