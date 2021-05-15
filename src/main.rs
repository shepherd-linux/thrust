//use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::fs::File;


fn main() -> std::io::Result<()> {
    let out : Value =  serde_json::from_reader( match File::open("config.json") {
        Ok(f) => f,
        Err(_) => panic!("Couldn't open config file.")
    })?;

    println!("{}", out["menu"].as_str().unwrap());
    Ok(())
}

//struct Config {
//    entries : Vec<Entry>,
//}
//
//#[derive(Deserialize)]
//struct Entry {
//    name : String,
//    args : Vec<String>,
//    tc   : Vec<u8>,
//}
//
//impl Config {
//    fn load(path : &str) -> () {
//        let parsed = parse_cfg(path);
//        
//    }
//}
//
//fn parse_cfg(path : &str){
//    // load JSON
//}