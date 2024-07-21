use std::fs::File;
use std::io::Write;
use serde_json::{json, Value};

pub fn recreate(file: &String) -> Result<(File, Value), String> {
    let instaces_list_default_struct: serde_json::Value = json!({
        "groups": {}
    });

    match File::create(file) {
        Ok(mut file) => {
            file.write_all(serde_json::to_string_pretty(&instaces_list_default_struct).unwrap().as_bytes()).unwrap();
            Ok((file, instaces_list_default_struct))
        },
        Err(e) => Err(format!("Failed to create instances list file: {}", e)),
    }
}