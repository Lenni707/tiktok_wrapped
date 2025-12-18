use serde_json::{Value};
use std::fs;
use std::collections::HashSet;

// -- TODO --
// > krasse Rekursion um immer deeper zu diggen
// > alle daten aufsplitten um einzeln zu verarbeiten
// > wenn bei alle geguckten videos, mit zeit in watchsessions aufteilen

fn main() {
    let json: String = fs::read_to_string("/Users/lenni/Downloads/tiktok_data.json").expect("wrong path dumbass");
    let v: Value = serde_json::from_str(&json).expect("didnt work");

    println!("{:?}", get_top_keys(v));
}

fn get_top_keys(v: Value) -> HashSet<String> {
    let map = v.as_object().expect("expect so abfuck");
    map.keys().cloned().collect() // .cloned macht aus &String -> String, .collect ist goated und bildet aus igerator (map.keys()), was festes wie HashSet (HashSet wegen Return type)
}
