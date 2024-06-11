use std::collections::{HashMap, HashSet};

use async_std::{fs::{create_dir_all, File}, io::WriteExt};
use serde_json::{self, json};

pub async fn download_version_libs<'a>(data: &serde_json::Value) -> Result<(&'a str, Vec<String>), String> {
    let paths = extract_manifest_libs(data, "linux").await;
    println!("Download Finished");
    Ok(("/home/quartix/.sonata/libraries/", paths))
}

async fn extract_manifest_libs(manifest: &serde_json::Value, current_os: &str) -> Vec<String> {
    // HashMap with next data: name, (path, url)
    let mut version_libs: HashMap<&str, (&str, &str)>= HashMap::new();

    println!("Iterating over manifest values...");

    if let Some(libraries) = manifest["libraries"].as_array() {
        for lib in libraries {

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
                                                    version_libs.insert(lib_name, (lib_path, lib_url));
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
                            version_libs.insert(lib_name, (lib_path, lib_url));
                        }
                    }
                }
            }
        }
    }

    return download_missing_libs(version_libs).await;
}


#[derive(Eq, Hash, PartialEq, Debug)]
struct LibInfo<'a> {
    name: &'a str,
    path: &'a str,
}
async fn download_missing_libs(version_libs: HashMap<&str, (&str, &str)>) -> Vec<String> {
    let metacache_file = std::fs::File::open("/home/quartix/.sonata/metacache.json").unwrap();
    let metacache: serde_json::Value = serde_json::from_reader(&metacache_file).unwrap();
    let mut downloaded_libs: HashSet<LibInfo> = HashSet::new();
    let mut libs_paths = Vec::new();

    if let Some(libraries) = metacache["libraries"].as_array() {
        for (k, v) in version_libs.iter() {

            let mut found = false;

            for lib in libraries {
                if let Some(lib_name) = lib["name"].as_str() {
                    if lib_name == *k {
                        //println!("Library found in metacache");
                        found = true;
                        break;
                    } else {
                        //println!("lib_name: {}, k: {}", lib_name, k);
                    }
                }
            }

            if !found {
                if let Some(pos) = v.0.rfind('/') {
                    println!("Cleared path: {}", &v.0[..pos]);

                    match create_dir_all(format!("/home/quartix/.sonata/libraries/{}", &v.0[..pos])).await {
                        Ok(_) => {},
                        Err(e) => {
                            println!("{e}");
                            break;
                        }
                    }

                    match surf::get(v.1).await {
                        Ok(mut response) => {
                            println!("Library \"{}\" not found in metacache, downloading...", k);
                            let mut file = File::create(format!("/home/quartix/.sonata/libraries/{}", &v.0)).await.unwrap();
                            async_std::io::copy(&mut response, &mut file).await.unwrap();
                            downloaded_libs.insert(LibInfo { name: *k, path: &v.0 });
                        }, 
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                } else {
                    println!("Failed to get last char");
                }
            }

            if libs_paths.is_empty() {
                libs_paths.push(format!("/home/quartix/.sonata/libraries/{}", &v.0));
            } else {
                libs_paths.push(format!(":/home/quartix/.sonata/libraries/{}", &v.0));
            }
        }

    }

    register_libs(downloaded_libs, metacache).await;
    libs_paths
}

async fn register_libs(downloaded_libs: HashSet<LibInfo<'_>>, mut metacache: serde_json::Value) {
    //let metacache_data = fs::read_to_string("/home/quartix/.sonata/metacache.json").await.unwrap();

    //if let Some(lib_pos) = metacache_data.match_indices("libraries").next() {
    //    println!("{:#?}", lib_pos.0);
    //    println!("{}", metacache_data[..lib_pos.0 + 11].to_string());

    //    if let Some(lib_array_pos) = metacache_data[lib_pos.0 + 11..].match_indices("[").next() {
    //        println!("{:#?}", lib_array_pos.0);
    //        println!("{}", metacache_data[..lib_pos.0 + 12 + lib_array_pos.0].to_string());

    //        for item in downloaded_libs.iter() {
    //            println!("{:#?}", item);
    //        }
    //    }
    //}

    if let Some(libs) = metacache["libraries"].as_array_mut() {
        for item in downloaded_libs.iter() {
            libs.push(json!({
                "name": item.name,
                "path": item.path,
            }));
        }
    }

    let mut metacache_file = File::create("/home/quartix/.sonata/metacache.json").await.unwrap();
    metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).await.unwrap();
}