use serde_json::Value;
use std::{
    collections::BTreeSet,
    fs,
    io::{self, Write},
};

fn collect_top_level_keys(v: &Value, keys: &mut BTreeSet<String>) {
    if let Value::Object(map) = v {
        keys.extend(map.keys().cloned());
    }
}

fn collect_child_keys(v: &Value, parent_key: &str, keys: &mut BTreeSet<String>) {
    let Some(child) = v.get(parent_key) else { return };

    match child {
        Value::Object(map) => keys.extend(map.keys().cloned()),
        Value::Array(arr) => {
            for item in arr {
                if let Value::Object(map) = item {
                    keys.extend(map.keys().cloned());
                }
            }
        }
        _ => {}
    }
}

fn find_values_by_key<'a>(v: &'a Value, target: &str) -> Vec<&'a Value> {
    let mut out = Vec::new();
    find_values_by_key_inner(v, target, &mut out);
    out
}

fn find_values_by_key_inner<'a>(v: &'a Value, target: &str, out: &mut Vec<&'a Value>) {
    match v {
        Value::Object(map) => {
            if let Some(val) = map.get(target) {
                out.push(val);
            }
            for child in map.values() {
                find_values_by_key_inner(child, target, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                find_values_by_key_inner(child, target, out);
            }
        }
        _ => {}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("/home/lenni/Downloads/user_data_tiktok.json")?;
    let v: Value = serde_json::from_str(&s)?;

    loop {
        print!("> ");
        io::stdout().flush()?; // show prompt

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // split into command + params
        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap();

        match cmd {
            "fetch" => {
                // optional subcommand
                match parts.next() {
                    None => {
                        // plain: fetch
                        let mut keys = BTreeSet::new();
                        collect_top_level_keys(&v, &mut keys);
                        for k in &keys {
                            println!("{k}");
                        }
                    }
                    Some("child") => {
                        let parent = parts.collect::<Vec<_>>().join(" ");
                        if parent.is_empty() {
                            eprintln!("usage: fetch child <TopLevelKey>");
                            continue;
                        }

                        let mut keys = BTreeSet::new();
                        collect_child_keys(&v, &parent, &mut keys);

                        if keys.is_empty() {
                            eprintln!("No child keys found for: {parent}");
                        } else {
                            for k in &keys {
                                println!("{k}");
                            }
                        }
                    }
                    Some("key") => {
                        let key = parts.collect::<Vec<_>>().join(" ");
                        if key.is_empty() {
                            eprintln!("usage: fetch key <KeyName>");
                            continue;
                        }

                        let values = find_values_by_key(&v, &key);
                        if values.is_empty() {
                            eprintln!("Key not found: {key}");
                        } else {
                            for (i, val) in values.iter().enumerate() {
                                println!("{i}: {key} = {val}");
                            }
                        }
                    }
                    _ => {
                        println!("no method found for this command");
                    }
                }

                // um besser lesen zu klnnen
                println!();
                println!();
            }
            _ => break, // if nithing then break
        }
    }

    Ok(())
}