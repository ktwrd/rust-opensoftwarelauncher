//use serde_json::{Result};
use tokio;

mod com;
use com::*;

mod fs;
use fs::*;

pub mod structs {
    pub mod connection;
    pub mod details;
}

use std::env::Args;
// internal crates
use crate::structs::{details::Details, connection::TokenResponse};
use crate::structs::connection::Connected;
use crate::structs::connection::Build;

#[tokio::main]
async fn main() {

    let d: Details = details_deser();

    match d.token.clone().len() {
        0 => {
        let token = osl_token_grant(d.clone()).await.data.token.token;
        println!("this program requires a token, please inser in details.json\n{:?}", token);
        return
        },
        _ => d.clone().token
    };
    

    let args = std::env::args();

    cli_engine(args, d).await
}


pub async fn cli_engine(mut args: Args, d: Details) {

    // the String "" is substituted for None (rust null) to avoid errors while matching 

    let null = "".to_string();

    let mut args3 = vec![];

    for x in 0..args.len() {
        let temp = args.nth(0);
        args3.push(temp.clone().unwrap_or(null.to_string()));

    }


    for _ in 0..10 {
        args3.push(null.clone());
    };


    match &args3[1] as &str {

        "--get" => cli_get(d).await,
        "--connect" => {let res = osl_connect(d.url).await; println!("Connected\nVersion: {}", res.version)},
        "--version" => cli_version(args3[2].to_string()).await,
        _ => {println!("Invalid Command\n--get, --connect"); return},
    };

}
pub async fn cli_get(d: Details) {

    println!("Establishing connection...");
    let release = osl_release(d).await; 
    println!("Connected!");
    write_rel(release);
    println!("Latest build info saved");


}
pub async fn cli_version(arg: String) {

    let builds = read_rel();

    println!("There are {} products, named:", builds.len());
    for x in 0..builds.len() {
        println!("{}", builds[x].productname)
    };

 
}
