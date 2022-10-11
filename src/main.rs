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
    

    let mut args = std::env::args();

    if args.len() < 3 {

        println!("Help\n\n--version (current, latest, streams) | shows the version given");
        return
    };


    let a1 = args.nth(1).unwrap();
    let a2 = args.nth(0).unwrap();

    match &a1 as &str {

        "--version" => { 
        
            let release = &osl_release(d.clone()).await;
            

            match &a2 as &str {

                "current" => println!("{}", osl_connect(d.url.clone()).await.version),

                "releases" => {
                    for x in 0..release.len() { 
                        println!("{:?}", release[x].ProductName)
                    };
                },
                "streams" => {
                    for x in 0..release[0].Streams.len() {
                    println!("Branch: {}\nVersion: {}",
                             release[0].Streams[x].BranchName, release[0].Streams[x].ProductVersion
                             );
                    };

                },

                             
                _ => println!("Options\n--releases, --streams")
            };
        },
        _ => println!("invalid Command"),
    };
}
