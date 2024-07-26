use async_std::fs::create_dir_all;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{
    Write,
    BufReader,
    Cursor
};

use serde_json::json;

use crate::utils::download::download;
use super::{Java, EntryInfo};


pub enum DownloadTypes {
    LZMA,
    Raw,
}

pub async fn download_java_part(url: &str, download_type: DownloadTypes, path: &String) -> Result<(), String> {
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

    let data = match download(url.to_string()).await {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    let file_buffer = match download_type {
        DownloadTypes::LZMA => {
            let mut data = BufReader::new(Cursor::new(data));
            let mut decomp = Vec::new();

            if let Err(e) = lzma_rs::lzma_decompress(&mut data, &mut decomp) {
                return Err(e.to_string());
            }

            decomp
        },
        DownloadTypes::Raw => {
            data
        }
    };

    file.write_all(&file_buffer).unwrap();

    println!("File writed: {}", path);

    Ok(())
}

pub async fn register_java(
    metacache: &mut serde_json::Value,
    metacache_file: String,
    java: &Java,
    downloaded_paths: HashSet<EntryInfo>,
    exec_path: &String
) -> Result<(), String> {
    if metacache["javas"].as_array_mut() == None || metacache["javas"].as_array_mut().unwrap().len() == 0 {
        metacache["javas"] = json!([]);

        let mut insertion = json!({
            "runtime-type": java.runtime_name,
            "version": java.version,
            "exec_path": exec_path,
            "sha1": java.sha1,
            "paths": []
        });

        for item in downloaded_paths {
            insertion["paths"].as_array_mut().unwrap().push(json!({
                "path": item.path,
                "path_type": item.path_type,
            }));
        }

        metacache["javas"].as_array_mut().unwrap().push(insertion);
    } else {
        if let Some(existing_javas) = metacache["javas"].as_array_mut() {

            for existing_java in existing_javas {
                if let Some(existing_sha1) = existing_java["sha1"].as_str() {
                    if existing_sha1 == java.sha1 {
                        if let Some(existing_paths) = existing_java["paths"].as_array_mut() {

                            for item in downloaded_paths {
                                existing_paths.push(json!({
                                    "path": item.path,
                                    "path_type": item.path_type,
                                }));
                            }
                        }

                        break;
                    }
                }
            }
        }
    }

    let mut metacache_file = File::create(&metacache_file).unwrap();
    metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).unwrap();

    Ok(())
}

pub async fn get_java_part
(
    downloads: serde_json::Map<String, serde_json::Value>,
    file_type: &str,
    path: &String,
) -> Result<EntryInfo, String> {
    let (download_type, url) = if let Some(file_url) = downloads.get("lzma").and_then(|v| v["url"].as_str()) {
        (DownloadTypes::LZMA, file_url)
    } else if let Some(file_url) = downloads.get("raw").and_then(|v| v["url"].as_str()) {
        (DownloadTypes::Raw, file_url)
    } else {
        return Err(format!("Failed to determine download type"));
    };

    match file_type {
        "file" => {
            match download_java_part(url, download_type, path).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            }

            return Ok(EntryInfo { path: path.to_string(), path_type: "file".to_string() })
        },
        "directory" => {
            match create_dir_all(&path).await {
                Ok(_) => (),
                Err(e) => return Err(e.to_string()),
            }

            return Ok(EntryInfo { path: path.to_string(), path_type: "directory".to_string() })
        },
        "link" => {
            return Ok(EntryInfo { path: path.to_string(), path_type: "link".to_string() })
        }
        _ => {
            println!("Unsupported file type");
            return Err("".to_string())
        },
    }
}
