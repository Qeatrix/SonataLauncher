use std::collections::HashMap;
use async_std::fs::create_dir_all;

pub mod download;
use download::libs;
use download::assets;
use download::manifest;

pub mod launch;

pub struct Instance<'a> {
    name: String,
    version: String,
    url: String,
    libs: HashMap<&'a str, (&'a str, &'a str)> 
}

#[derive(Debug)]
pub struct LaunchArgs {
    username: String,
    version: String,
    game_dir: String,
    assets_dir: String,
    asset_index: String,
    libs_dir: Vec<String>,
    uuid: String,
    access_token: String,
    client_id: String,
    x_id: String,
    user_type: String,
    version_type: String,
    main_class: String,
}

impl Instance<'_> {
    pub fn new<'a>(name: String, version: String, url: String) -> Instance<'a> {
        Instance {
            name,
            version,
            url,
            libs: HashMap::new()
        }
    }

    pub async fn init(&self) -> Result<String, String> {
        // Get minecraft version manifest
        let verson_manifest: serde_json::Value;
        match manifest::download_manifest(&self.url).await {
            Ok(data) => verson_manifest = data,
            Err(e) => return Err(format!("Failed to download version manifest: {}", e))
        }

        // Download all libs needed by this version
        let (_libs_dir, lib_paths) = match libs::download_version_libs(&verson_manifest).await {
            Ok((dir, paths)) => {
                (dir.to_string(), paths)
            },
            Err(e) => return Err(format!("Failed to download version manifest: {}", e))
        };

        // Get version assets manifest
        let (assets_manifest, asset_index) = match manifest::get_assets_manifest(&verson_manifest).await {
            Ok((data, id)) => {
                (data, id.to_string())
            },
            Err(e) => return Err(format!("Failed to download assets manifest: {}", e))
        };

        // Download all assets needed by this version
        let assets_dir = assets::download_version_assets(&assets_manifest).await;

        // Initialize instance directory
        match Self::init_instance_dir(&self.name).await {
            Ok(_) => {},
            Err(e) => return Err(format!("Failed to initialize instance directory: {}", e))
        };

        let args = LaunchArgs {
            username: String::from("Melicta"),
            version: self.version.clone(),

            game_dir: String::from("/home/quartix/.sonata/instances"),

            assets_dir, 
            asset_index,

            uuid: String::from(""),
            access_token: String::from(""),
            client_id: String::from(""),
            x_id: String::from(""),
            user_type: String::from(""),
            version_type: String::from("release"),

            libs_dir: lib_paths,
            main_class: String::from("net.minecraft.client.main.Main"),
        };

        launch::launch_instance(verson_manifest, args).await;

        Ok(format!("asd"))
    }

    async fn init_instance_dir(name: &String) -> Result<(), String> {
        match create_dir_all(format!("/home/quartix/.sonata/instances/{}", name)).await {
            Ok(_) => {
                println!("Created instance dir");
                
                let instances_list_file = std::fs::File::open("/home/quartix/.sonata/instances/instances_list.json").unwrap();
                let instances_list: serde_json::Value = serde_json::from_reader(&instances_list_file).unwrap();

                if let Some(groups) = instances_list["groups"].as_object() {
                    if let Some(vanilla_groups) = groups["Vanilla"].as_array() {
                        for group in vanilla_groups {
                            if let Some(name) = group["name"].as_str() {
                                println!("Name: {}", name);
                            }
                        }

                    } else {
                        println!("")
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