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


#[tokio::main]
async fn main() {

    let d = details_deser();
    let url = format!("https://{}", d.url);

    let connection = osl_connect_deser(osl_connect(url).await);

    println!("Connected to build server successfully\nUptime: {}\nVersion: {}", 
             connection.Uptime, connection.Version);



}

