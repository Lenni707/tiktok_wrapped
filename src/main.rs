use serde_json::{Value};
use std::fs;
use std::collections::HashSet;

// -- TODO --
// > krasse Rekursion um immer deeper zu diggen
// > alle daten aufsplitten um einzeln zu verarbeiten
// > wenn bei alle geguckten videos, mit zeit in watchsessions aufteilen

fn main() {
    let json: String = fs::read_to_string("tiktok_data.json").expect("wrong path dumbass");
    let v: Value = serde_json::from_str(&json).expect("didnt work");

    println!("{:?}", get_top_keys(&v));
    
    // ghet nicht ka wrm
    let watched_vids = v
        .get("Profile")
        .and_then(|p| p.get("Profile Map")); // iwie brauche ich goofy as hell closures || für bestimte impls (Kapitel 13 rust buch)

    println!("{:?}", watched_vids)
}

fn get_top_keys(v: &Value) -> HashSet<String> {
    let map = v.as_object().expect("expect so abfuck");
    map.keys().cloned().collect() // .cloned macht aus &String -> String, .collect ist goated und bildet aus igerator (map.keys()), was festes wie HashSet (HashSet wegen Return type)
}

// fn get_specific_values() -> Vec<_> {

// }
