use std::fs;
use std::fs::File;

use std::{
    io::{Read, Write},
};

use crate::structs::details::Details;
use crate::Build;
use std::process::Command;


pub async fn osl_install(payload: String) {

    let _ = Command::new("wget")
        .args(["-q", &payload])
        .output();
    println!("Finished");



}

pub fn file_init() {

    let mut file = File::create("details.json")
        .expect("file could not be created");

}


pub fn write_details() -> Details {


    let mut json = 
            b"{
    \"url\": \"\",
    \"username\": \"\",
    \"password\": \"\",
    \"token\": \"\"
}";

    let mut file = File::create("details.json")
        .expect("Failed to open details.json");

    file.write_all(json);

    panic!("Please insert details to details.json")
}

pub fn details_deser() -> Details {
    
    
    let mut file_test = File::open("details.json");

    match file_test {
        Ok(_) => {},
        Err(_) => file_init(),
    };

    let mut details_file = File::open("details.json")
        .expect("Failed to open file details.json");


    let mut details_str = String::new();

    details_file.read_to_string(&mut details_str)
        .expect("Failed writing details.json");


    let dets = serde_json::from_str(&details_str);

    let details = match dets {

        Ok(_) => dets.unwrap(),
        Err(e) => write_details(),
    };
        

    return details

}


pub fn write_rel(rel: Vec<Build>) {
    
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
