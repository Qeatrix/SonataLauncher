use async_std::task::{self, JoinHandle};
use std::collections::HashSet;
use futures::stream::FuturesUnordered;
use futures::StreamExt;

use super::{
    Java,
    EntryInfo,
    arch,
    download,
};

use crate::utils::download::download_in_json;


pub async fn start_extraction(metacache_path: String, metacache_value: &mut serde_json::Value, java: &mut Java) -> Result<String, String> {
    let (java_manifest, local_paths): (serde_json::Value, Vec<String>) = match parse_main_manifest(&metacache_value, java).await {
        Ok(data) => {
            if let Some(cdata) = data.0 {
                cdata
            } else {
                if let Some(exec_path) = data.1 {
                    return Ok(exec_path);
                }

                return Err(format!("Failed to download or find existing java instance"));
            }
        },
        Err(e) => return Err(e),
    };

    match parse_java_manifest(java_manifest, &java, metacache_path, metacache_value, local_paths).await {
        Ok(data) => {
            if let Some(exec_file) = data {
                // println!("Downloaded");
                return Ok(exec_file);
            } else {
                return Err(format!("Failed to find executable file"));
            }
        },
        Err(e) => return Err(e),
    };
}

async fn process_futures
(
    futures: &mut FuturesUnordered<JoinHandle<Option<EntryInfo>>>,
    downloaded_paths: &mut HashSet<EntryInfo>,
) {
    while let Some(result) = futures.next().await {
        if let Some(entry_info) = result {
            downloaded_paths.insert(entry_info);
        }
    }
}

pub async fn parse_java_manifest(
    manifest: serde_json::Value,
    java: &Java,
    metacache_path: String,
    metacache: &mut serde_json::Value,
    local_paths: Vec<String>
) -> Result<Option<String>, String> {
    let mut exec_file = None;
    let mut downloaded_paths: HashSet<EntryInfo> = HashSet::new();
    let mut futures: FuturesUnordered<JoinHandle<Option<EntryInfo>>> = FuturesUnordered::new();

    if let Some(files) = manifest["files"].as_object() {
        for file in files {
            let raw_path = file.0.to_string();
            let global_path = format!("{}/{}", java.destination, &raw_path);

            // Check for executable file
            if let Some(raw_path_last_point) = raw_path.rfind('/') {
                let file_name = &raw_path[raw_path_last_point + 1..];
                if let Some(is_exec) = file.1["executable"].as_bool() {
                    if is_exec == true && file_name == "java"  {
                        exec_file = Some(global_path.clone());
                    }
                }
            }

            if let Some(path_type) = file.1["type"].as_str() {

                // Check for existing path in metacache
                let mut found = false;
                if local_paths.len() > 0 {
                    for local_path in &local_paths {
                        let raw_path = format!("{}/java/", metacache_path[..metacache_path.rfind('/').unwrap()].to_string());
                        let path = &local_path[raw_path.len()..].to_string();

                        // We need to get a local path from global path to compare with manifest path
                        if path == file.0 {
                            found = true;
                            break
                        }
                    }
                }

                if found {
                    continue;
                }

                if let Some(downloads) = file.1["downloads"].as_object() {
                    let downloads = downloads.to_owned();
                    let path_type = path_type.to_string();

                    futures.push(task::spawn(async move {
                        match download::get_java_part(downloads, &path_type, &global_path).await {
                            Ok(data) => Some(data),
                            Err(_e) => None,
                        }
                    }));

                    if futures.len() >= 100 {
                        process_futures(&mut futures, &mut downloaded_paths).await;
                    }
                } else {
                    if path_type == "link" || path_type == "directory" {
                        downloaded_paths.insert(EntryInfo { path: global_path, path_type: path_type.to_string() });
                    }
                }
            }
        }
    }

    process_futures(&mut futures, &mut downloaded_paths).await;

    if let Some(exec_file) = exec_file {
        match download::register_java(metacache, metacache_path, java, downloaded_paths, &exec_file).await {
            Ok(data) => data,
            Err(e) => return Err(e),
        };

        Ok(Some(exec_file))
    } else {
        Ok(None)
    }
}

async fn parse_main_manifest
(
    metacache: &serde_json::Value,
    java: &mut Java
) -> Result<
        (Option<(serde_json::Value, Vec<String>)>, Option<String>),
    String>
{
    let manifest = match download_in_json(java.manifest_url.to_string()).await {
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

                        // Check java installation
                        if let Some(java_sha1) = runtime[0]["manifest"]["sha1"].as_str() {
                            java.sha1 = java_sha1.to_string();
                            let mut local_paths = Vec::new();

                            if let Some(javas) = metacache["javas"].as_array() {
                                for java in javas {
                                    if let Some(downloaded_java_sha1) = java["sha1"].as_str() {
                                        if downloaded_java_sha1 == java_sha1 {
                                           if let Some(paths) = java["paths"].as_array() {
                                               // println!("{}", paths.len());
                                               for path in paths {
                                                   if let Some(inner_path) = path["path"].as_str() {
                                                       local_paths.push(inner_path.to_string());
                                                   }
                                               }
                                           }

                                            // if let Some(exec_path) = java["exec_path"].as_str() {
                                            //     println!("This java is already installed");
                                            //     return Ok((None, Some(exec_path.to_string())));
                                            // }

                                            break;
                                        }
                                    }
                                }
                            }

                            match download_in_json(manifest_url.to_string()).await {
                                Ok(data) => {
                                    return Ok(
                                        (Some(
                                            (data, local_paths)
                                        ), None)
                                    );
                                },
                                Err(e) => return Err(e),
                            };
                        }
                    }
                }
            }
        }
    }

    Err(format!("Failed to parse JSON of available Javas"))
}
