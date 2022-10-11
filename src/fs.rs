use serde_json::{Result};

use std::{
    io::{Read, Write},
    fs::{File},
};
use crate::structs::details::Details;
use crate::structs::connection::Connected;
use crate::TokenResponse;

pub fn details_deser() -> Details {
    
    let mut details_json = File::open("details.json")
        .expect("File \"details.json\" not found");

    let mut details_str = String::new();

    details_json.read_to_string(&mut details_str)
        .expect("Failed writing details.json");

    let details: Details = serde_json::from_str(&details_str).expect("Failed to deser details.json");

    return details
}
