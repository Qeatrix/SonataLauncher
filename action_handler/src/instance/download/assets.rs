use std::{collections::HashSet, io::Write};
use async_std::{
    fs::{create_dir_all, File}, io::WriteExt, task
};
use futures::{stream::FuturesUnordered, StreamExt};
use serde_json::json;
use tide_websockets::WebSocketConnection;

use crate::types::ws::{send_ws_msg, ProgressData, ProgressMessage, ProgressTarget};

use super::manifest::is_array_exists;


pub async fn download_version_assets<'a>(manifest: &serde_json::Value, assets_path: &'a str, ws: &WebSocketConnection) {
    extract_manifest_assets(manifest, assets_path, ws).await;
    println!("Asset extraction completed");
    // ws.send_string(format!("Assets downloaded")).await;

    // let msg = ProgressFinishMessage {
    //     message_id: format!("stage_complete"),
    //     message_type: "PROGRESS_FINISH".to_string(),
    //     timestamp: format!("Current Date"),
    //     data: ProgressFinishData {
    //         stage: "download_assets".to_string(),
    //         status: "COMPLETED".to_string(),
    //     },
    // };

    let msg = ProgressMessage {
        message_id: format!("stage_complete"),
        timestamp: format!("Current Date"),
        data: ProgressData {
            stage: "download_assets".to_string(),
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
        println!("Failed to send update info, {e}");
    }
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct AssetInfo {
    name: String,
    hash: String,
}

async fn extract_manifest_assets<'a>(manifest: &'a serde_json::Value, assets_path: &str, ws: &WebSocketConnection) {
    let base_url = "https://resources.download.minecraft.net/";
    let metacache_file_path = "/home/quartix/.sonata/metacache.json";
    let metacache_file = std::fs::File::open("/home/quartix/.sonata/metacache.json").unwrap();
    let mut metacache: serde_json::Value = serde_json::from_reader(&metacache_file).unwrap();
    let mut downloaded_assets: HashSet<AssetInfo> = HashSet::new();

    if !is_array_exists(&metacache, "assets") {
        if let Some(metacache_object) = metacache.as_object_mut() {
            metacache_object.insert("assets".to_string(), json!([]));
            let mut metacache_file = std::fs::File::create(metacache_file_path).unwrap();
            metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).unwrap();
        }
    }

    if let Some(objects) = manifest["objects"].as_object() {
        if let Some(assets) = metacache["assets"].as_array() {
            let mut futures = FuturesUnordered::new();

            println!("Checking for assets...");

            for (k, v) in objects {
                if !assets.iter().any(|asset| {
                    asset["name"].as_str() == Some(k) && asset["hash"].as_str() == v["hash"].as_str()
                }) {
                    let base_url = base_url.to_string();
                    let hash = v["hash"].as_str().unwrap().to_string();
                    let name = k.to_string();
                    let assets_path = assets_path.to_string();

                    futures.push(task::spawn(async move {
                        println!("Downloading asset '{}'", name);
                        match download_asset(&base_url, &hash, &name, &assets_path).await {
                            Ok(asset_info) => {
                                Some(asset_info)
                            },
                            Err(e) => {
                                println!("{e}");
                                None
                            }
                        }
                    }));

                    if futures.len() >= 100 {
                        process_futures(&mut futures, &mut downloaded_assets, objects.len(), ws).await;
                    }
                }
            }

            process_futures(&mut futures, &mut downloaded_assets, objects.len(), ws).await;
        }
    }

    register_assets(downloaded_assets, metacache).await;
}

async fn process_futures
(
    futures: &mut FuturesUnordered<async_std::task::JoinHandle<std::option::Option<AssetInfo>>>,
    downloaded_assets: &mut HashSet<AssetInfo>,
    max: usize,
    ws: &WebSocketConnection
) {
    while let Some(result) = futures.next().await {
        if let Some(asset_info) = result {
            let msg = ProgressMessage {
                message_id: format!("download_item_complete"),
                timestamp: format!("Current Date"),
                data: ProgressData {
                    stage: "download_assets".to_string(),
                    determinable: true,
                    progress: Some(downloaded_assets.len()),
                    max,
                    status: "INPROGRESS".to_string(),
                    target_type: "FILE".to_string(),
                    target: ProgressTarget::File {
                        status: "DOWNLOADED".to_string(),
                        name: asset_info.name.to_string(),
                        size_bytes: 0,
                    },
                },
            };

            if let Err(e) = send_ws_msg(ws, json!(msg)).await {
                println!("Failed to send update info, {e}");
            }
            downloaded_assets.insert(asset_info);
        }
    }
}

async fn download_asset(base_url: &str, asset_hash: &str, asset_name: &str, path: &str) -> Result<AssetInfo, String> {
    let asset_url_data = construct_asset_url(&base_url, &asset_hash.to_string());
    let ( full_url, hash_part, hash ) = asset_url_data;

    match create_dir_all(format!("{}/{}", path, hash_part)).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to create dir for asset: {e}");
            return Err(e.to_string());
        }
    }

    match surf::get(&full_url).await {
        Ok(mut response) => {
            let mut file = File::create(
                    format!("{}/{}/{}",
                    path,
                    hash_part,
                    hash)
                ).await.unwrap();
            async_std::io::copy(&mut response, &mut file).await.unwrap();

            Ok(AssetInfo { name: asset_name.to_string(), hash: asset_hash.to_string() })
        }, 
        Err(e) => {
            println!("{e}");
            Err(e.to_string())
        }
    }
}

async fn register_assets(downloaded_assets: HashSet<AssetInfo>, mut metacache: serde_json::Value) {
    if let Some(assets) = metacache["assets"].as_array_mut() {
        for item in downloaded_assets.iter() {
            assets.push(json!({
                "name": item.name,
                "hash": item.hash,
            }));
        }
    }

    let mut metacache_file = File::create("/home/quartix/.sonata/metacache.json").await.unwrap();
    metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).await.unwrap();
}

fn construct_asset_url(base_url: &str, hash: &String) -> (String, String, String) {
    let hash_part = &hash[0..2].to_string();

    (
        format!("{}{}/{}", base_url, hash_part, hash),
        hash_part.to_string(),
        hash.to_string()
    )
}