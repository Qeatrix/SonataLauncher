use std::{collections::HashSet, io::Write};
use async_std::{
    fs::{create_dir_all, File},
    io::WriteExt, task
};
use futures::{stream::FuturesUnordered, StreamExt};
use serde_json::json;

use super::manifest::is_array_exists;


pub async fn download_version_assets(manifest: &serde_json::Value) -> String {
    extract_manifest_assets(manifest).await;
    println!("Asset extraction completed");

    String::from("/home/quartix/.sonata/assets/")
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct AssetInfo {
    name: String,
    hash: String,
}

async fn extract_manifest_assets<'a>(manifest: &'a serde_json::Value) {
    let base_url = "https://resources.download.minecraft.net/";
    let metacache_file = std::fs::File::open("/home/quartix/.sonata/metacache.json").unwrap();
    let mut metacache: serde_json::Value = serde_json::from_reader(&metacache_file).unwrap();
    let mut downloaded_assets: HashSet<AssetInfo> = HashSet::new();

    match is_array_exists(&metacache, "assets") {
        true => {},
        false => {
            if let Some(metacache_object) = metacache.as_object_mut() {
                metacache_object.insert("assets".to_string(), json!([]));

                let mut metacache_file = std::fs::File::create("/home/quartix/.sonata/metacache.json").unwrap();
                metacache_file.write_all(serde_json::to_string_pretty(&metacache).unwrap().as_bytes()).unwrap();

                metacache = serde_json::from_reader(&metacache_file).unwrap();
            }
        }
    }

    if let Some(objects) = manifest["objects"].as_object() {
        if let Some(assets) = metacache["assets"].as_array() {
            let mut futures = FuturesUnordered::new();

            println!("Checking for assets...");

            for (k, v) in objects {

                let mut found = false;

                if assets.len() > 0 {
                    for asset in assets {
                        if let Some(asset_name) = asset["name"].as_str() {
                            if let Some(asset_hash) = asset["hash"].as_str() {
                                // If asset name or asset hash is not the same as in metacache, download it
                                if asset_name == *k && asset_hash == v.get("hash").unwrap().as_str().unwrap() {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                if found {
                    continue;
                }

                let base_url = base_url.to_string();
                let hash = v["hash"].as_str().unwrap().to_string();
                let name = k.to_string();

                futures.push(task::spawn(async move {
                    println!("Downloading asset '{}'", name);
                    match download(&base_url, &hash, &name).await {
                        Ok(asset_info) => Some(asset_info),
                        Err(e) => {
                            println!("{e}");
                            None
                        }
                    }
                }));

                if futures.len() >= 20 {
                    while let Some(result) = futures.next().await {
                        if let Some(asset_info) = result {
                            downloaded_assets.insert(asset_info);
                        }
                    }
                }
            }

            while let Some(result) = futures.next().await {
                if let Some(asset_info) = result {
                    downloaded_assets.insert(asset_info);
                }
            }
        }
    }

    register_assets(downloaded_assets, metacache).await;
}

async fn download(base_url: &str, asset_hash: &str, asset_name: &str) -> Result<AssetInfo, String> {
    let asset_url_data = construct_asset_url(&base_url, &asset_hash.to_string());
    let ( full_url, hash_part, hash ) = asset_url_data;

    match create_dir_all(format!("/home/quartix/.sonata/assets/{}", hash_part)).await {
        Ok(_) => {},
        Err(e) => {
            println!("{e}");
        }
    }

    match surf::get(&full_url).await {
        Ok(mut response) => {
            let mut file = File::create(
                    format!("/home/quartix/.sonata/assets/{}/{}",
                    &hash_part,
                    &hash)
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
    let hash_part = &hash[1..3].to_string();
    let hash = hash[1..hash.len() - 1].to_string();

    (
        format!("{}{}/{}", base_url, hash_part, hash),
        hash_part.to_string(),
        hash
    )
}