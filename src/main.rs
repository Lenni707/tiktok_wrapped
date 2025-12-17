use serde_json::*;
use std::fs;

fn main() {
    let json: String = fs::read_to_string("/home/lenni/Downloads/user_data_tiktok.json").expect("wrong path dumbass");
    let v: Value = serde_json::from_str(&json).expect("didnt work");

    match v {
        Value::Array(map) => {
            
        }
        Value::Object(map) => {

        }
        _ => {return}
    }
}