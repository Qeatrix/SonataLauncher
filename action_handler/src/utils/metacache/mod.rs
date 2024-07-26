use std::fs::{File, OpenOptions};
use std::io::Write;
use serde_json::{json, Value};


pub fn recreate(file_path: &String) -> Result<(File, Value), String> {
    let metacache_default_struct: serde_json::Value = json!({
        "javas": [],
        "libraries": [],
        "assets": []
    });

    match File::create(file_path) {
        Ok(mut file) => {
            file.write_all(serde_json::to_string_pretty(&metacache_default_struct).unwrap().as_bytes()).unwrap();
            Ok((file, metacache_default_struct))
        },
        Err(e) => Err(format!("Failed to create metacache file: {}", e)),
    }
}

pub fn get_json(path: &String) -> Result<Value, String> {
    let metacache_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path).unwrap();

    match serde_json::from_reader(&metacache_file) {
        Ok(value) => return Ok(value),
        Err(_) => {
            match recreate(&path) {
                Ok((_file, value)) => return Ok(value),
                Err(e) => {
                    println!("Failed to recreate metacache file: {}", e);
                    return Err(format!("Failed to recreate metacache file: {e}"));
                },
            }
        }
    };
}
