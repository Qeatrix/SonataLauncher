use std::collections::HashMap;
use std::fs::OpenOptions;
use chrono::format;
use home::home_dir;
use std::io::{ErrorKind, Write};
use async_std::fs::create_dir_all;

pub mod download;
use async_std::{fs::File, io::WriteExt};
use download::libs;
use download::assets;
use download::manifest;
use serde_json::json;
use tide_websockets::WebSocketConnection;

pub mod launch;

use crate::types::ws::send_ws_msg;
use crate::types::ws::InfoMessage;
use crate::types::ws::ProgressData;
use crate::types::ws::ProgressMessage;
use crate::types::ws::ProgressTarget;
use crate::types::ws::ProgressTargetsList;
use crate::utils::instances_list;

pub struct Paths {
    root: String,
    libs: String,
    assets: String,
    instances: String,
    instance: String,
    instances_list_file: String,
    meta: String,
    metacache_file: String,
}

pub struct Instance {
    name: String,
    url: String,
    info: HashMap<String, String>
}

impl Instance {
    pub fn new(name: String, url: String, info: HashMap<String, String>) -> Instance {
        Instance {
            name,
            url,
            info
        }
    }

    pub async fn init(&mut self, ws: &WebSocketConnection) -> Result<String, String> {
        // Get default paths
        let paths = match get_required_paths(&self.name) {
            Ok(paths) => paths,
            Err(e) => return Err(e),
        };

        match create_dir_all(&paths.root).await {
            Ok(_) => {
                println!("Launcher root directory initialized");

                let msg = InfoMessage {
                    message: format!("Root directory initialized successfully"),
                    message_id: format!("creation_root_success"),
                    message_type: format!("INFO"),
                    timestamp: format!("Current Date"),
                };

                if let Err(e) = send_ws_msg(ws, json!(msg)).await {
                    println!("Error occured: {}", e);
                    return Err(e);
                }
            },
            Err(e) => {
                return Err(format!("{}", e));
            },
        };

        self.info.entry("${game_directory}".to_string()).or_insert_with(|| paths.instance.to_string());
        self.info.entry("${assets_root}".to_string()).or_insert_with(|| paths.assets.to_string());
        self.info.entry("${user_properties}".to_string()).or_insert_with(|| "{}".to_string());

        // Send stages list
        let mut list: Vec<String> = Vec::new();
        list.push("fetch_manifest".to_string());
        list.push("download_libs".to_string());
        list.push("download_assets".to_string());

        let msg = ProgressTargetsList {
            message_id: format!("progress_targets_list_transfer"),
            message_type: format!("PROGRESS_TARGETS_LIST"),
            timestamp: format!("Current Date"),
            ids_list: list,
        };

        if let Err(e) = send_ws_msg(ws, json!(msg)).await {
            println!("Error occured: {}", e);
            return Err(e);
        }


        // Get minecraft version manifest
        let verson_manifest: serde_json::Value;
        match manifest::download_manifest(&self.url, &paths.meta).await {
            Ok(data) => {
                let msg = ProgressMessage {
                    message_id: format!("stage_complete"),
                    timestamp: format!("Current Date"),
                    data: ProgressData {
                        stage: "fetch_manifest".to_string(),
                        determinable: false,
                        progress: None,
                        max: 0,
                        status: "COMPLETED".to_string(),
                        target_type: "".to_string(),
                        target: ProgressTarget::File {
                            status: "".to_string(),
                            name: "".to_string(),
                            size_bytes: 0,
                        },
                    },
                };

                if let Err(e) = send_ws_msg(ws, json!(msg)).await {
                    return Err(e);
                }

                verson_manifest = data
            },
            Err(e) => return Err(format!("Failed to download version manifest: {}", e))
        }

        // Download all libs needed by this version
        match libs::download_version_libs(&verson_manifest, &paths, &ws).await {
            Ok((dir, paths)) => {
                let mut paths_string = String::new();

                for path in paths.iter() {
                    paths_string.push_str(&path);
                }

                self.info.insert("${libs_directory}".to_string(), dir.to_string());
                self.info.insert("${classpath_libs_directories}".to_string(), paths_string);
            },
            Err(e) => return Err(format!("Failed to download and register libs: {e}"))
        };

        println!("Libs downloaded");

        // Get version assets manifest
        let assets_manifest_location = paths.assets.to_owned() + "/indexes";
        let assets_manifest = match manifest::get_assets_manifest(&verson_manifest, &assets_manifest_location).await {
            Ok((data, id)) => {
                self.info.insert("${assets_index_name}".to_string(), id.to_string());
                data
            },
            Err(e) => return Err(format!("Failed to download assets manifest: {}", e))
        };

        // Download all assets needed by this version
        let assets_objects_location = paths.assets.to_owned() + "/objects";
        assets::download_version_assets(&assets_manifest, &assets_objects_location, ws, &paths).await;

        // Initialize instance directory
        match Self::init_instance_dir(&self.name, &paths).await {
            Ok(_) => {},
            Err(e) => return Err(format!("Failed to initialize instance directory: {}", e))
        };

        println!("asd");
        launch::launch_instance(verson_manifest, &self.info).await;

        Ok(format!("asd"))
    }

    async fn init_instance_dir(name: &String, paths: &Paths) -> Result<(), String> {
        match create_dir_all(&paths.instance).await {
            Ok(_) => {
                println!("Created instance dir");
                
                let instances_list_file = match OpenOptions::new()
                    .read(true)
                    .write(false)
                    .create(false)
                    .open(&paths.instances_list_file) {
                    Ok(file) => {
                        println!("Instances list found");
                        file
                    },

                    Err(e) => {
                        if e.kind() == ErrorKind::NotFound {
                            match instances_list::recreate(&paths.instances_list_file) {
                                Ok(_) => {
                                    match OpenOptions::new()
                                        .read(true)
                                        .write(false)
                                        .create(false)
                                        .open(&paths.instances_list_file) {
                                            Ok(file) => file,
                                            Err(e) => return Err(e.to_string()),
                                        }
                                },

                                Err(e) => return Err(format!("Failed to create instances list file: {}", e))
                            }
                        } else {
                            return Err(format!("Failed to open instances list file: {}", e));
                        }
                    }
                };

                let mut instances_list: serde_json::Value = serde_json::from_reader(&instances_list_file).unwrap();

                if let Some(groups) = instances_list["groups"].as_object_mut() {
                    if let Some(vanilla_item) = groups.get_mut("Vanilla") {
                        if let Some(vanilla) = vanilla_item.as_array_mut() {
                            for item in vanilla.iter() {
                                println!("{:#?}", item.get("name"));
                                if let Some(item_name) = item.get("name") {
                                    if item_name == name {
                                        println!("Existing item found");
                                        return Ok(());
                                    }
                                }
                            }
                            
                            vanilla.push(json!({
                                "name": name,
                                "path": paths.instance,
                            }));
                        }
                    } else {
                        groups.insert("Vanilla".to_string(), json!([{
                            "name": name,
                            "path": paths.instance,
                        }]));
                    }

                    let mut instances_list_file = File::create(&paths.instances_list_file).await.unwrap();
                    instances_list_file.write_all(serde_json::to_string_pretty(&instances_list).unwrap().as_bytes()).await.unwrap();
                } else {
                    return Err(format!("\"groups\" object not found in instances list file."));
                }

                Ok(())
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
}

// Returnes Libs path, Assets path, Instances path
fn get_required_paths(instance_name: &String) -> Result<Paths, String> {
    let homedir = match home_dir() {
        Some(path) => path,
        None => return Err("Failed to get home directory".to_string()),
    };

    let root = format!("{}/.sonata", homedir.display());

    Ok(Paths {
        root: root.to_string(),
        libs: format!("{}/libraries", root),
        assets: format!("{}/assets", root),
        instances: format!("{}/instances", root),
        instance: format!("{}/instances/{}", root, instance_name),
        instances_list_file: format!("{}/instances/instances_list.json", root),
        meta: format!("{}/meta", root),
        metacache_file: format!("{}/metacache.json", root),
    })
}