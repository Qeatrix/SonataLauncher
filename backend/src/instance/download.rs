use std::{collections::HashMap, fs::create_dir_all};

use serde_json::Value;

const LAUNCHER_ROOT: &str = "/mnt/drive0/sonata/";

pub async fn download_main_manifest() -> Result<Value, String> {
  let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
  match reqwest::get(url).await {
    Ok(response) => {
      match response.json::<Value>().await {
        Ok(data) => Ok(data),
        Err(e) => Err(format!("Failed to parse JSON: {}", e)),
      }
    },

    Err(e) => {
      Err(format!("Failed to download manifest file: {}", e))
    }
  }
}

pub async fn download_version_manifest<'a>(url: String) -> Result<&'a str, String> {
    match reqwest::get(url).await {
        Ok(response) => {
            match response.json::<Value>().await {
                Ok(data) => {
                    run_libs_downloader(data, "linux").await;
                    println!("Download Finished");
                    Ok("Download Finished")
                },
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        },

        Err(e) => {
            Err(format!("Failed to download manifest file: {}", e))
        }
    }
}

async fn run_libs_downloader(manifest: Value, current_os: &str) {
    let mut to_download: HashMap<&str, (&str, &str)>= HashMap::new();

    println!("Iterating over manifest values...");

    if let Some(libraries) = manifest["libraries"].as_array() {
        for lib in libraries {
            println!("{:#?}", lib);

            if let Some(rules) = lib["rules"].as_array() {
                for rule in rules {

                    if let Some(action) = rule["action"].as_str() {
                        if action == "allow" {

                            if let Some(os) = rule["os"].as_object() {
                                if let Some(os_name) = os["name"].as_str() {
                                    if os_name == current_os {

                                        if let Some(lib_name) = lib["name"].as_str() {
                                            if let Some(lib_path) = lib["downloads"]["artifact"]["path"].as_str() {
                                                if let Some(lib_url) = lib["downloads"]["artifact"]["url"].as_str() {
                                                    to_download.insert(lib_name, (lib_path, lib_url));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                if let Some(lib_name) = lib["name"].as_str() {
                    if let Some(lib_path) = lib["downloads"]["artifact"]["path"].as_str() {
                            if let Some(lib_url) = lib["downloads"]["artifact"]["url"].as_str() {
                                to_download.insert(lib_name, (lib_path, lib_url));
                            }
                    }
                }
            }
        }
    }

    println!("Iteration finished, result:");
    println!("{:#?}", to_download);
}

pub fn init_root() -> Result<(), String> {
    match create_dir_all(LAUNCHER_ROOT) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to initialize root dir: {}", e))
    }
}