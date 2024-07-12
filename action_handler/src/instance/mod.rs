use std::collections::HashMap;
use std::io::ErrorKind;
use async_std::fs::create_dir_all;

pub mod download;
use async_std::fs::File;
use download::libs;
use download::assets;
use download::manifest;
use serde_json::json;
use tide_websockets::WebSocketConnection;

pub mod launch;

use crate::types::ws;
use crate::types::ws::send_ws_msg;
use crate::types::ws::InfoMessage;
use crate::types::ws::ProgressData;

pub struct Paths {
    root: String,
    libs: String,
    assets: String,
    instance: String,
    meta: String,
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
        let paths = get_required_paths(&self.name);

        match create_dir_all(&paths.root).await {
            Ok(_) => {
                println!("Launcher root directory initialized");

                let msg = InfoMessage {
                    message: format!("Root directory initialized successfully"),
                    message_id: format!("creation_root_success"),
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

        // Get minecraft version manifest
        let verson_manifest: serde_json::Value;
        match manifest::download_manifest(&self.url, &paths.meta).await {
            Ok(data) => {
                let msg = InfoMessage {
                    message: format!("Manifest downloaded successfully"),
                    message_id: format!("download_manifest_success"),
                    timestamp: format!("Current Date"),
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
        assets::download_version_assets(&assets_manifest, &assets_objects_location, ws).await;

        // Initialize instance directory
        // match Self::init_instance_dir(&self.name, &paths).await {
        //     Ok(_) => {},
        //     Err(e) => return Err(format!("Failed to initialize instance directory: {}", e))
        // };

        println!("asd");
        launch::launch_instance(verson_manifest, &self.info).await;

        Ok(format!("asd"))
    }

    async fn init_instance_dir(name: &String, paths: &Paths) -> Result<(), String> {
        match create_dir_all(format!("{}/{}", paths.instance, name)).await {
            Ok(_) => {
                println!("Created instance dir");
                
                let instances_list_file = match std::fs::File::open("/home/quartix/.sonata/instances/instances_list.json") {
                    Ok(file) => {
                        println!("Instances list found");
                        file
                    },

                    Err(e) => {
                        if e.kind() == ErrorKind::NotFound {
                            match File::create(format!("{}/instances/instances_list.json", paths.root)).await {
                                Ok(_) => {
                                    match std::fs::File::open(format!("{}/instances/instances_list.json", paths.root)) {
                                        Ok(file) => file,
                                        Err(e) => return Err(format!("Failed to create instances list file: {}", e)),
                                    }
                                },

                                Err(e) => return Err(format!("Failed to create instances list file: {}", e))
                            }
                        } else {
                            return Err(format!("Failed to open instances list file: {}", e));
                        }
                    }
                };

                let instances_list: serde_json::Value = serde_json::from_reader(&instances_list_file).unwrap();

                if let Some(groups) = instances_list["groups"].as_object() {
                    if let Some(vanilla_groups) = groups["Vanilla"].as_array() {
                        for group in vanilla_groups {
                            if let Some(name) = group["name"].as_str() {
                                println!("Name: {}", name);
                            }
                        }

                    } else {
                        println!("");
                    }
                } else {
                    println!("Not found");
                }

                Ok(())
            },
            Err(e) => {
                return Err(format!("Failed to create instance dir: {}", e));
            }
        }
    }
}

// Returnes Libs path, Assets path, Instances path
fn get_required_paths(instance_name: &String) -> Paths {
    let root = "/home/quartix/.sonata";

    Paths {
        root: root.to_string(),
        libs: format!("{}/libraries", root),
        assets: format!("{}/assets", root),
        instance: format!("{}/{}", root, instance_name),
        meta: format!("{}/meta", root),
    }
}