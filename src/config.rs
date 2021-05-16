use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::File;

pub fn get_config() -> Result<Config, serde_json::Error> {
    let out: Value = serde_json::from_reader(match File::open("config.json") {
        Ok(f) => f,
        Err(_) => panic!("Couldn't open config file."),
    })?;
    serde_json::from_value(out)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    pub name: String,
    pub fields: Vec<String>,
    pub termination_codes: Vec<u8>,
    pub name_field_arg: String,
}
