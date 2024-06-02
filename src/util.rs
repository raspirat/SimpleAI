use serde_json::Value;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_json_file(path: &Path) -> std::io::Result<Value> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json: Value = serde_json::from_str(&contents)?;

    Ok(json)
}
