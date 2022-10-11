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
    
    println!("OpenSoftwareLauncher");

    // temp
    let url = String::from("http://buildservice.api.minalogger.com");

    let connection_in = osl_example(url).await;

    println!("Server found!");

    let connection = osl_connect_deser(connection_in);

    println!("Connected to build server successfully\nUptime: {}\nVersion: {}", connection.Uptime, connection.Version);

}

