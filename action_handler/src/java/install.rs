use async_std::fs::create_dir_all;
use serde_json::json;
use std::{collections::HashSet, io::{BufReader, Cursor, Write}, fs::{OpenOptions, File}};

use crate::{instance::download::manifest, utils::{download::{self, download_in_json}, metacache::{self, recreate}}};

use super::{arch, Java};


enum DownladTypes {
    LZMA,
    Raw,
}

pub async fn init(java: &mut Java, metacache_path: String) -> Result<String, String> {
    let metacache_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&metacache_path).unwrap();

    let metacache: serde_json::Value = match serde_json::from_reader(&metacache_file) {
        Ok(value) => value,
        Err(_) => {
            match recreate(&metacache_path) {
                Ok((_file, value)) => value,
                Err(e) => {
                    println!("Failed to recreate metacache file: {}", e);
                    return Err(format!("Failed to recreate metacache file: {e}"));
                },
            }
        }
    };


    let java_manifest: serde_json::Value = match parse_main_manifest(&metacache, java).await {
        Ok(data) => {
            if let Some(json_value) = data.0 {
                json_value
            } else {
                if let Some(exec_path) = data.1 {
                    return Ok(exec_path);
                }

                return Err(format!("Failed to download or find existing java instance"));
            }
        },
        Err(e) => return Err(e),
    };

    match parse_java_manifest(java_manifest, &java, metacache_path, metacache).await {
        Ok(data) => {
            if let Some(exec_file) = data {
                return Ok(exec_file);
            } else {
                return Err(format!("Failed to find executable file"));
            }
        },
        Err(e) => return Err(e),
    };
}

async fn register_java(
    mut metacache: serde_json::Value,
    metacache_file: String,
    java: &Java,
    downloaded_paths:
    HashSet<EntryInfo>,
    exec_path: &String
) -> Result<(), String> {
    if metacache["javas"].as_array_mut() == None {
        metacache["javas"] = json!([]);
    }

    let mut new_java_insertion = json!({
        "runtime-type": java.runtime_name,
        "version": java.version,
        "exec_path": exec_path,
        "sha1": java.sha1,
        "paths": []
    });

    for item in downloaded_paths {
        new_java_insertion["paths"].as_array_mut().unwrap().push(json!({
            "path": item.path,
            "path_type": item.path_type,
        }));
    }

    metacache["javas"].as_array_mut().unwrap().push(new_java_insertion);

    let mut metacache_file = File::create(&metacache_file).unwrap();
    metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).unwrap();

    Ok(())
}

async fn get_java_part(download_type: DownladTypes, url: &str, file_type: &str, path: &String) -> Result<(), String> {
    match file_type {
        "file" => {
            let path_to_file = &path[..path.rfind('/').unwrap()];
            let file_name = &path[path.rfind('/').unwrap() + 1..];

            match create_dir_all(path_to_file).await {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            };

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path_to_file.to_owned() + "/" + file_name).unwrap();

            let file_buffer = match download_type {
                DownladTypes::LZMA => {
                    let mut file_data = match download::download(url.to_string()).await {
                        Ok(data) => BufReader::new(Cursor::new(data)),
                        Err(e) => return Err(e),
                    };

                    let mut decomp = Vec::new();
                    lzma_rs::lzma_decompress(&mut file_data, &mut decomp).unwrap();
                    decomp
                },
                DownladTypes::Raw => {
                    match download::download(url.to_string()).await {
                        Ok(data) => data,
                        Err(e) => return Err(e),
                    }
                }
            };

            file.write_all(&file_buffer).unwrap();

            return Ok(())
        },
        "directory" => {
            match create_dir_all(&path).await {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }

            return Ok(())
        }
        _ => {
            println!("Unsupported file type");
            return Ok(())
        },
    }
}


#[derive(Eq, Hash, PartialEq, Debug)]
struct EntryInfo {
    path: String,
    path_type: String,
}

async fn parse_java_manifest(
    manifest: serde_json::Value,
    java: &Java,
    metacache_path: String,
    metacache: serde_json::Value
) -> Result<Option<String>, String> {
    let mut exec_file = None;
    let mut downloaded_paths: HashSet<EntryInfo> = HashSet::new();

    if let Some(files) = manifest["files"].as_object() {
        for file in files {
            if let Some(path_type) = file.1["type"].as_str() {
                let raw_path = file.0.to_string();
                let global_path = format!("{}/{}", java.destination, &raw_path);

                if let Some(file_url) = file.1["downloads"]["lzma"]["url"].as_str() {
                    match get_java_part(DownladTypes::LZMA, file_url, path_type, &global_path).await {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    };
                } else if let Some(file_url) = file.1["downloads"]["raw"]["url"].as_str() {
                    match get_java_part(DownladTypes::Raw, file_url, path_type, &global_path).await {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }

                if let Some(raw_path_last_point) = raw_path.rfind('/') {
                    let file_name = &raw_path[raw_path_last_point + 1..];
                    if let Some(is_exec) = file.1["executable"].as_bool() {
                        if path_type == "file" && is_exec == true && file_name == "java"  {
                            exec_file = Some(global_path.clone());
                        }
                    }
                }

                downloaded_paths.insert(EntryInfo { path: global_path, path_type: path_type.to_string() });
            }
        }
    }

    if let Some(exec_file) = exec_file {
        match register_java(metacache, metacache_path, java, downloaded_paths, &exec_file).await {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        Ok(Some(exec_file))
    } else {
        Ok(None)
    }
}

async fn parse_main_manifest(metacache: &serde_json::Value, java: &mut Java) -> Result<(Option<serde_json::Value>, Option<String>), String> {
    let manifest = match download::download_in_json(java.manifest_url.to_string()).await {
        Ok(data) => data,
        Err(e) => return Err(e.to_string()),
    };

    let osarch = match arch::manifest_osarch() {
        Some(data) => data,
        None => {
            let e = String::from("Your current OS or CPU architecture is not supported");
            return Err(e);
        }
    };

    if let Some(macos) = manifest[osarch].as_object() {
        if let Some(runtime) = macos[&java.runtime_name].as_array() {
            if let Some(version_name) = runtime[0]["version"]["name"].as_str() {
                let version_name = version_name.to_string();
                let major_version = &version_name[..version_name.find('.').unwrap()];
                if major_version == java.version {
                    if let Some(manifest_url) = runtime[0]["manifest"]["url"].as_str() {

                        if let Some(java_sha1) = runtime[0]["manifest"]["sha1"].as_str() {
                            java.sha1 = java_sha1.to_string();

                            if let Some(javas) = metacache["javas"].as_array() {
                                for java in javas {
                                    if let Some(downloaded_java_sha1) = java["sha1"].as_str() {
                                        if downloaded_java_sha1 == java_sha1 {
                                            if let Some(exec_path) = java["exec_path"].as_str() {
                                                println!("This java is already installed");
                                                return Ok((None, Some(exec_path.to_string())));
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        match download_in_json(manifest_url.to_string()).await {
                            Ok(data) => return Ok((Some(data), None)),
                            Err(e) => return Err(e),
                        };
                    }
                }
            }
        }
    }

    Err(format!("Failed to parse JSON of available Javas"))
}
