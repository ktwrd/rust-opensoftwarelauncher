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
use crate::structs::connection::{Build, BuildStream};
use crate::structs::connection::ProductFileResponse;


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

        "--get"     => cli_get(d).await,
        "--connect" => {let res = osl_connect(d.url).await; println!("Connected\nVersion: {}", res.version)},
        "--version" => cli_version(args3[2].to_string()).await,
        "--info"    => cli_info(args3, d.clone()).await,
        "--install" => cli_install(args3, d.clone()).await,
        _ => {println!("Invalid Command\n--get, --connect"); return},
    };

}

pub async fn cli_install(arg3: Vec<String>, d: Details) {

    let builds = read_rel();



    if arg3[2] == "" {
        println!("No matches found (try running --version to see avaiable software)");
        return
    };

   
    osl_install(arg3[2].clone()).await;

}




pub async fn cli_info(arg3: Vec<String>, d: Details) {

    let p = read_rel();

    for x in 0..p.len() {
        if p[x].productname == arg3[2] {

            let stream = streams(arg3.clone(), p[x].streams.clone(), d.clone()).await;
            
            let res = match &arg3[3] as &str {

                "--ProductID" | "--productid" => &p[x].productid,
                "--streams"   | "--Streams"   => &stream,
                _                         => "Options: (--streams, --productid)",
            };

            println!("{res}");
        
        };
    };


}

pub async fn streams(arg3: Vec<String>, s: Vec<BuildStream>, d: Details) -> String {

    if arg3[4] == "" {
        
        let mut res = String::new();

        for x in 0..s.len() {

            let newstring = format!("ProductName {}\nProductVersion: {}\nBranchName: {}", 
                                    s[x].productname, s[x].productversion, s[x].branchname);

            res += &format!("{}\n\n", newstring);
        };

        return res
        
    }else {

        let stre = arg3[4].clone();

        let res = match &arg3[4].clone() as &str {
            "nightly" | "Nightly" => &s[0].commithash,
            "beta"    | "Beta"    => &s[0].commithash,
            "stable"  | "Stable"  => &s[0].commithash,
            _ => panic!("Stream not found"),
        };

       let streamf = osl_file(d.url, res.to_string(), d.token).await;

       println!("There are {} files associated with Stream {}", streamf.len(), stre);

       for x in 0..streamf.len() {

           println!("location: {}", streamf[x].location);
       };

       return String::new()

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
