use crate::structs::details::Details;
use crate::TokenResponse;
use crate::Connected;
use crate::Build;
use crate::ProductFileResponse;

pub async fn osl_redeem(key: String, d: Details) {

    let link = format!("{}/license/redeem?token={}&key={key}", d.url, d.token);

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .get(&link)
        .send().await;

    let cli_res = match client {
        Ok(_) => client.unwrap(),
        Err(e) => {println!("{e}"); return}
    };

    if cli_res.status() != 200 {
        println!("{}", cli_res.status()); return
    }
    println!("{}", cli_res.text().await.unwrap());



}
pub async fn osl_file(url: String, hash: String, token: String) -> Vec<ProductFileResponse> {

    let link = format!("{url}/file?hash={hash}&token={token}");

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .get(&link)
        .send().await;

     let cli_res = match client {
         Ok(_) => client.unwrap(),
         Err(e) => panic!("{e}")
     };

     if cli_res.status() != 200 {
         panic!("{}", cli_res.status());
     }

     let result: Vec<ProductFileResponse> = 
         serde_json::from_str(&cli_res.text().await.unwrap())
         .expect("failed to deserialize ProductFileResponse");

     
     return result


}

pub async fn osl_connect(url: String) -> Connected {

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .get(&url)
        .send().await;

    let cli_res = match client {
        Ok(_) => client.unwrap(),
        Err(e) => panic!("{e}")
     };

    if cli_res.status() != 200 {
        panic!("{}", cli_res.status());
    }

      
    let result: Connected = serde_json::from_str(&cli_res.text().await.unwrap())
        .expect("failed to deserialize connection request");

    return result

}


pub async fn osl_token_grant(d: Details) -> TokenResponse  {


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

     let result: TokenResponse = serde_json::from_str(&cli_res)
         .expect("failed to deserialize token response");

     return result

}

pub async fn osl_release(d: Details) -> Vec<Build> {

    
    let link = format!("{}/release/latest/com.minalyze.minalogger?token={}", d.url, d.token);

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
        reqwest::Client::new()
        .get(&link)
        .send().await;

    let cli_res = match client {
         Ok(_) => client.unwrap(),
         Err(e) => panic!("{e}")
     };

    if cli_res.status() != 200 {
         panic!("{}", cli_res.status());
     }

    let result: Vec<Build> = serde_json::from_str(&cli_res.text().await.unwrap())
         .expect("failed to deserialize Build Vec");

    return result


}
