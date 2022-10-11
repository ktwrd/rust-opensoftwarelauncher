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


#[tokio::main]
async fn main() {

    let d: Details = details_deser();

    println!("{:?}", d);
    let connection = osl_connect_deser(osl_connect(d.url.clone()).await);

    println!("Connected to build server successfully\nUptime: {}\nVersion: {}", 
             connection.Uptime, connection.Version);

    

    let token1 = osl_token_grant(d).await;

    println!("{:?}", token1);
}

