
use std::fs;
use std::fs::File;

use std::{
    io::{Read, copy},
};

use std::io;

use reqwest::Response;

use crate::structs::details::Details;
use crate::Build;



pub fn details_deser() -> Details {
    
    let mut details_json = File::open("details.json")
        .expect("File \"details.json\" not found");

    let mut details_str = String::new();

    details_json.read_to_string(&mut details_str)
        .expect("Failed writing details.json");

    let details: Details = serde_json::from_str(&details_str).expect("Failed to deser details.json");

    return details
}


pub fn write_rel(rel: Vec<Build>) {
    
    let mut file = File::open("release.json")
        .expect("File \"release.json\" not found");

    let mut out = String::new();

    for x in 0..rel.len() {
        let current: String = serde_json::to_string(&rel[x]).unwrap();
        out = format!("{out}\n{current}");
    };

    out = format!("[{out}]");

    fs::write("release.json", out).expect("Unable to write file");
}

pub fn read_rel() -> Vec<Build> {

    let mut file = File::open("release.json")
         .expect("File \"release.json\" not found");

    let mut out = String::new();

    file.read_to_string(&mut out);

    if out == "\n" {
        panic!("No data found in release.json (try running --get)");
    
    }else {

        let build: Vec<Build> = serde_json::from_str(&out)
            .expect("Failed to deser release.json");

        return build
    }

}
