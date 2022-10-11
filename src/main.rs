use serde_json::{Result};
use tokio;

mod com;
use com::*;

mod fs;
use fs::*;

pub mod structs {
    pub mod connection;
}
// internal crates


#[tokio::main]
async fn main() {
    
    // temp
    let url = String::from("http://buildservice.api.minalogger.com");

    let connection = osl_connect_deser(osl_connect(url).await);

    println!("Connected to build server successfully\nUptime: {}\nVersion: {}", 
             connection.Uptime, connection.Version);




}

