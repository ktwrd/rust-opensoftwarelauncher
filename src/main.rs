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
    

    let mut args = std::env::args();

    cli_engine(args, d).await
}


pub async fn cli_engine(mut args: Args, d: Details) {

    // the String "" is substituted for None (rust null) to avoid errors while matching 

    let null = "".to_string();
   
  
    let mut args2 = vec![];
    for x in 0..args.len() {
        args2.push(args.nth(0))
    };

    let mut args3 = vec![];

    for x in 0..args2.len()  {
        args3.push(args2[x].as_ref().unwrap_or(&null));
    };

    for x in 0..10 {
        args3.push(&null);
    };


    match &args3[1] as &str {

        "--get" => cli_get(d).await,
        "--connect" => {let res = osl_connect(d.url).await; println!("Connected\nVersion: {}", res.version)},
        "--version" => cli_version(d, args3[2].to_string()).await,
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
pub async fn cli_version(d: Details, arg: String) {

    let builds = read_rel();
 
    println!("Product: {}", builds[0].productname)
}

/*
    if args.len() < 3 {

        println!("Minimum of two paramaters for CLI");
        return
    };

   

    //let a1 = args.nth(1).unwrap();
    //let a2 = args.nth(0).unwrap();

    let mut args2 = vec![None, None, None, None, None];
    for x in 0..args.len() {
        args2[x] = args.nth(0);
    };

    println!("{:?}", args2);

    
    match &args2[0]?  {

    
     "a"   => {},

       // "--version" => { 
       // 
       //     let release = &osl_release(d.clone()).await;
     //   },
    };
}
            

            match &args2[1] {

  
                "current" => println!("{}", osl_connect(d.url.clone()).await.version),
            };
        },
    };
}

                "releases" => {
                    for x in 0..release.len() { 
                        println!("{:?}", release[x].productname)
                    };
                },
                Some("streams") => {
                    for x in 0..release[0].streams.len() {
                    println!("Branch: {}\nVersion: {}",
                             release[0].streams[x].branchname, release[0].streams[x].productversion
                             );
                    };

              },

                             
                _ => println!("Invalid Command\n--version (current, latest, streams)")
            };
        },
        _ => println!("invalid Command"),
    };
}
*/
