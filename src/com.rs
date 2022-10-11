use serde_json::{Result};

pub async fn osl_example(url: String) -> String {


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
