use serde_json::{Result};
use tokio;

mod com;
use com::*;

#[tokio::main]
async fn main() {
    
    println!("OpenSoftwareLauncher");

    // temp
    let url = String::new();

    let connection = osl_example(url).await;

    println!("server found");

}

