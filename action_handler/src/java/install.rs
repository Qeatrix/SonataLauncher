use async_std::fs::{create_dir_all, OpenOptions};
use futures::AsyncWriteExt;
use std::io::{Cursor, BufReader};

use crate::utils::download::{self, download_in_json};

use super::{arch, Java};


enum DownladTypes {
    LZMA,
    Raw,
}

pub async fn init(java: &Java) -> Result<(), String> {
    let java_manifest = match parse_main_manifest(&java).await {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    parse_java_manifest(java_manifest, &java.destination).await.unwrap();

    Ok(())
}

async fn get_java_part(download_type: DownladTypes, url: &str, file_type: &str, path: String) -> Result<(), String> {
    match file_type {
        "file" => {
            let path_to_file = &path[..path.rfind('/').unwrap()];
            let file_name = &path[path.rfind('/').unwrap() + 1..];

            match create_dir_all(path_to_file).await {
                Ok(_) => {
                    println!("Created path: {}", path);
                },
                Err(e) => return Err(e.to_string()),
            };

            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(path_to_file.to_owned() + "/" + file_name)
                .await.unwrap();

            let file_buffer = match download_type {
                DownladTypes::LZMA => {
                    println!("Using LZMA extraction type");
                    let mut file_data = match download::download(url.to_string()).await {
                        Ok(data) => BufReader::new(Cursor::new(data)),
                        Err(e) => return Err(e),
                    };

                    println!("{}", file_name);

                    let mut decomp = Vec::new();
                    lzma_rs::lzma_decompress(&mut file_data, &mut decomp).unwrap();
                    decomp
                },
                DownladTypes::Raw => {
                    println!("Using direct extraction type");
                    match download::download(url.to_string()).await {
                        Ok(data) => data,
                        Err(e) => return Err(e),
                    }
                }
            };

            file.write_all(&file_buffer).await.unwrap();
        },
        "directory" => {
            match create_dir_all(&path).await {
                Ok(_) => {
                    println!("Created path: {}", path);
                },
                Err(e) => return Err(e.to_string()),
            }
        }
        _ => {
            println!("Unsupported file type");
        },
    }

    Ok(())
}

async fn parse_java_manifest(manifest: serde_json::Value, java_path: &String) -> Result<(), String> {
    if let Some(files) = manifest["files"].as_object() {
        for file in files {
            if let Some(file_type) = file.1["type"].as_str() {
                let raw_path = file.0.to_string();
                let global_path = format!("{}/{}", java_path, &raw_path);
                if file.0 == "jre.bundle/Contents/Home/lib/jvm.cfg" {
                    println!("{}", file.0);
                }

                if let Some(file_url) = file.1["downloads"]["lzma"]["url"].as_str() {
                    get_java_part(DownladTypes::LZMA, file_url, file_type, global_path).await.unwrap();
                } else if let Some(file_url) = file.1["downloads"]["raw"]["url"].as_str() {
                    get_java_part(DownladTypes::Raw, file_url, file_type, global_path).await.unwrap();

                }
            }
        }
    }

    Ok(())
}

async fn parse_main_manifest(java: &Java) -> Result<serde_json::Value, String> {
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
                        println!("Found Java manifest: {}", manifest_url);
                        match download_in_json(manifest_url.to_string()).await {
                            Ok(data) => return Ok(data),
                            Err(e) => return Err(e),
                        };
                    }
                }
            }
        }
    }

    Err(format!("Failed to parse JSON of available Javas"))
}
