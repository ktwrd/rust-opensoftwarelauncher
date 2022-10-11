use serde_json::{Result};

use std::{
    io::{Read, Write},
    fs::{File},
};
use crate::structs::details::Details;
use crate::structs::connection::Connected;
use crate::TokenResponse;


pub fn osl_connect_deser(input: String) -> Connected {

    let result: Connected = serde_json::from_str(&input).expect("failed to deserialize connection request");

    return result


}

pub fn details_deser() -> Details {
    
    let mut details_json = File::open("details.json")
        .expect("File \"details.json\" not found");

    let mut details_str = String::new();

    details_json.read_to_string(&mut details_str)
        .expect("Failed writing details.json");

    let details: Details = serde_json::from_str(&details_str).expect("Failed to deser details.json");

    return details
}

pub fn token_response_deser(input: String) -> TokenResponse {

    let result: TokenResponse = serde_json::from_str(&input).expect("failed to deserialize token response");


    return result
}

/*
pub fn write_token(input: String) {


    // open file
    let mut file = File::open("details.json");
    
    let mut original_str = String::new();

    file.as_ref().unwrap().read_to_string(&mut original_str);

    let o: Details = serde_json::from_str(&original_str).unwrap();

    let new = Details {
        url: o.url,
        username: o.username,
        password: o.password,
        token: input
    };

    println!("{:?}", new);
    
    let returner = serde_json::to_string(&new);
    file.unwrap().write(returner.unwrap());



}
*/
