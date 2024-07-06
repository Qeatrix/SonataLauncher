use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::process::exit;
use std::io::Write;
use std::fs::OpenOptions;
use async_std::{fs::{create_dir_all, File}, io::WriteExt};
use serde_json::{self, json};

use crate::instance::Paths;

pub async fn download_version_libs<'a>(manifest: &serde_json::Value, paths: &Paths) -> Result<(&'a str, Vec<String>), String> {
    let paths = extract_manifest_libs(manifest, "linux", paths).await;
    println!("Download Finished");
    Ok(("/home/quartix/.sonata/libraries/", paths))
}

async fn extract_manifest_libs(manifest: &serde_json::Value, current_os: &str, paths: &Paths) -> Vec<String> {
    // Hashmap contains: hash, (name, path, url)
    let mut version_libs: HashMap<&str, (String, String, &str)> = HashMap::new();

    println!("Extraction libraries...");

    if let Some(libraries) = manifest["libraries"].as_array() {
        for lib in libraries {
            let lib_name = lib["name"].as_str();

            let allow_lib = if let Some(rules) = lib["rules"].as_array() {
                rules.iter().any(|rule| {
                    if let Some(action) = rule["action"].as_str() {
                        if action == "allow" {
                            if let Some(os) = rule["os"].as_object() {
                                if os.get("name").and_then(|name| name.as_str()) == Some(current_os) {
                                    return true;
                                }
                            } else {
                                return true;
                            }
                        }
                    }

                    false
                })
            } else {
                true
            };
            
            if allow_lib {
                let lib_path = lib["downloads"]["artifact"]["path"].as_str();
                let lib_url = lib["downloads"]["artifact"]["url"].as_str();
                let lib_hash = lib["downloads"]["artifact"]["sha1"].as_str();

                if let (Some(lib_name),
                        Some(lib_path),
                        Some(lib_url),
                        Some(lib_hash)) = (lib_name, lib_path, lib_url, lib_hash) {
                    version_libs.insert(lib_hash, (lib_name.to_string(), lib_path.to_string(), lib_url));
                }
            }


            // Check for classifiers
            if let Some(natives) = lib["natives"].as_object() {
                for (k, native_name) in natives {
                    if k == current_os {
                        if let Some(classifiers) = lib["downloads"]["classifiers"].as_object() {
                            for (name, v) in classifiers {
                                if name == native_name {
                                    let lib_path = v["path"].as_str();
                                    let lib_url = v["url"].as_str();
                                    let lib_hash = v["sha1"].as_str();
                                    
                                    if let (Some(lib_name),
                                            Some(lib_path),
                                            Some(lib_url),
                                            Some(lib_hash)) = (lib_name, lib_path, lib_url, lib_hash) {
                                        println!("Found: {}", lib_name);
                                        if let Some(updated) =
                                            version_libs.insert(lib_hash, (lib_name.to_string(), lib_path.to_string(), lib_url)) {

                                            println!("REWRITED: {:#?}", updated);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
            }
        }
    }


    if let Some(client_url) = manifest["downloads"]["client"]["url"].as_str() {
        let name = manifest["id"].as_str().unwrap();
        let name = name.to_owned() + "-client.jar";
        let path = "com/mojang/minecraft/".to_owned() + &name;
        let hash = manifest["downloads"]["client"]["sha1"].as_str().unwrap();

        version_libs.insert(hash, (name, path, client_url));
    }

    println!("{:#?}", version_libs);
    download_missing_libs(version_libs, paths).await
}


#[derive(Eq, Hash, PartialEq, Debug)]
struct LibInfo<'a> {
    hash: &'a str,
    name: &'a str,
    path: &'a str,
}
async fn download_missing_libs(version_libs: HashMap<&str, (String, String, &str)>, paths: &Paths) -> Vec<String> {
    let metacache_file_path = format!("{}/metacache.json", paths.root.to_owned());

    let mut metacache_file = OpenOptions::new()
                                            .read(true)
                                            .write(true)
                                            .create(true)
                                            .open(metacache_file_path).unwrap();


    let metacache: serde_json::Value = match serde_json::from_reader(&metacache_file) {
        Ok(value) => value,
        Err(_) => {
            let initial_json = json!({ "libraries": [] });
            metacache_file.write_all(serde_json::to_string_pretty(&initial_json).unwrap().as_bytes()).unwrap();

            match serde_json::from_reader(metacache_file) {
                Ok(value) => value,
                Err(e) => {
                    println!("{}", e);
                    exit(1)
                },
            }
        }
    };

    let mut downloaded_libs: HashSet<LibInfo> = HashSet::new();
    let mut libs_paths = Vec::new();

    if let Some(libraries) = metacache["libraries"].as_array() {
        for (k, v) in version_libs.iter() {
            let lib_hash = k;
            let lib_name = &v.0;
            let lib_path = &v.1;
            let lib_url = v.2;

            let found = libraries.iter().any(|lib| lib["hash"].as_str() == Some(lib_hash));

            if !found {
                if let Some(pos) = lib_path.rfind('/') {
                    let dir_path = format!("{}/{}", paths.libs, lib_path[..pos].to_string());
                    println!("Creating directory: {}", dir_path);

                    if let Err(e) = create_dir_all(&dir_path).await {
                        println!("Failed to create directory: {e}");
                        continue;
                    }

                    match surf::get(lib_url).await {
                        Ok(mut response) => {
                            println!("Downloading library \"{}\"", lib_name);
                            let file_path = format!("{}/{}", paths.libs, lib_path);
                            let mut file = File::create(&file_path).await.unwrap();
                            async_std::io::copy(&mut response, &mut file).await.unwrap();
                            downloaded_libs.insert(LibInfo { hash: lib_hash, name: lib_name, path: lib_path });
                        }
                        Err(e) => println!("Failed to download library: {e}"),
                    }
                } else {
                    println!("Failed to parse path: {}", v.0);
                }
            }

            if libs_paths.is_empty() {
                libs_paths.push(format!("{}/{}", paths.libs, lib_path));
            } else {
                libs_paths.push(format!(":{}/{}", paths.libs, lib_path));
            }

        }
    }

    register_libs(downloaded_libs, metacache).await;
    libs_paths
}

async fn register_libs(downloaded_libs: HashSet<LibInfo<'_>>, mut metacache: serde_json::Value) {
    if let Some(libs) = metacache["libraries"].as_array_mut() {
        for item in downloaded_libs.iter() {
            libs.push(json!({
                "hash": item.hash,
                "name": item.name,
                "path": item.path,
            }));
        }
    }

    let mut metacache_file = File::create("/home/quartix/.sonata/metacache.json").await.unwrap();
    metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).await.unwrap();
}