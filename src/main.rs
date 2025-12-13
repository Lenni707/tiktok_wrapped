use serde_json::Value;
use std::{collections::BTreeSet, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("/home/lenni/Downloads/user_data_tiktok.json")?;
    let v: Value = serde_json::from_str(&s)?;


    // get all headers (key names)
    let mut keys = BTreeSet::new();
    collect_key_names(&v, &mut keys);

    // get specifc value of key
    let key = "userName";
    if let Some(val) = find_value_by_key(&v, key) {
        println!("{key} = {val}");
    } else {
        eprintln!("key doesnt exist");
    }
    Ok(())
}

fn collect_key_names(v: &Value, keys: &mut BTreeSet<String>) {
    match v {
        Value::Object(map) => {
            for (k, child) in map {
                keys.insert(k.clone());
                collect_key_names(child, keys);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                collect_key_names(child, keys);
            }
        }
        _ => {}
    }
}

pub fn find_value_by_key<'a>(v: &'a Value, target: &str) -> Option<&'a Value> {
    match v {
        Value::Object(map) => {
            // wenn es existiert returnen, sonst deeper suchen
            if let Some(val) = map.get(target) {
                return Some(val);
            }
            // krasse rekursion OMG
            for child in map.values() {
                if let Some(found) = find_value_by_key(child, target) {
                    return Some(found);
                }
            }
            None
        }
        // Value von dem key nehmen
        Value::Array(arr) => {
            for child in arr {
                if let Some(found) = find_value_by_key(child, target) {
                    return Some(found);
                }
            }
            None
        }
        _ => None,
    }
}