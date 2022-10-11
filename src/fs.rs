use serde_json::{Result};

use std::{
    io::Read,
    fs::File,
};

use crate::structs::connection::Connected;


pub fn osl_connect_deser(input: String) -> Connected {

    let result: Connected = serde_json::from_str(&input).expect("failed to deserialize response");

    return result


}
