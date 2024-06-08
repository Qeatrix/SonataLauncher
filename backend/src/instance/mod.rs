use std::collections::HashMap;
use async_std::task::block_on;
use download::download_main_manifest;
use serde_json;

pub mod download;

pub struct Instance<'a> {
    name: String,
    version: String,
    libs: HashMap<&'a str, (&'a str, &'a str)> 
}

impl Instance<'_> {
    pub fn init(name: String, version: String) -> Result<String, String> {
        let mut libs: HashMap<&str, (&str, &str)> = HashMap::new();
        match block_on(download_main_manifest()) {
            Ok(value) => {
            },

            Err(e) => {

            }
        }

        let instance = Instance {
            name,
            version,
            libs
        };

        Ok(format!("asd"))
    }

    async fn download_main_manifest() -> Result<serde_json::Value, String> {
        let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
        match reqwest::get(url).await {
            Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
            },

            Err(e) => {
            Err(format!("Failed to download manifest file: {}", e))
            }
        }
    }

    async fn download_version_manifest<'a>(url: String) -> Result<&'a str, String> {
        match reqwest::get(url).await {
            Ok(response) => {
                match response.json::<serde_json::Value>().await {
                    Ok(data) => {
                        Self::run_libs_downloader(data, "linux").await;
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

    async fn run_libs_downloader(manifest: serde_json::Value, current_os: &str) {
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
}