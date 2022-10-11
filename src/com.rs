use serde_json::{Result};
use crate::structs::details::Details;


pub async fn osl_connect(url: String) -> String {


     let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
     .get(url.to_string())
         .send().await;

     let cli_res = match client {
         Ok(_) => client.unwrap().text().await.unwrap(),
         Err(e) => panic!("{e}")
     };

     return cli_res

}


pub async fn osl_token_grant(d: Details) -> String  {


    let (url, username, password) = (d.url, d.username, d.password);

    let payload = format!("{url}/token/grant?username={username}&password={password}");


    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
     .get(&payload)
     .send().await;

     let cli_res = match client {
         Ok(_) => client.unwrap().text().await.unwrap(),
         Err(e) => panic!("{e}")
     };

     return cli_res

}

pub async fn osl_release(d: Details, op: String) -> String {
   
       let (url, username, password) = (d.url, d.username, d.password);

    let payload = format!("{url}/release/latest/com.minalyze.minalogger");

     let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
     .get(&payload)
     .send().await;

     let cli_res = match client {
         Ok(_) => client.unwrap().text().await.unwrap(),
         Err(e) => panic!("{e}")
     };

     return cli_res
}
