pub async fn download_manifest(url: &String) -> Result<serde_json::Value, String> {
    match surf::get(url).await {
        Ok(mut response) => {
            match response.body_json::<serde_json::Value>().await {
                Ok(data) => {
                    println!("Version manifest downloaded");
                    Ok(data)
                },
                Err(e) => Err(format!("Failed to parse JSON: {}", e)),
            }
        },

        Err(e) => {
            Err(format!("Failed to download manifest file: {}", e))
        }
    }
}

pub async fn get_assets_manifest(version_manifest: &serde_json::Value) -> Result<(serde_json::Value, &str), String> {
    if let Some(asset_index) = version_manifest["assetIndex"].as_object() {
        if let Some(asset_url) = asset_index["url"].as_str() {
            match download_manifest(&asset_url.to_string()).await {
                Ok(manifest) => return Ok((manifest, asset_index["id"].as_str().unwrap())),
                Err(e) => return Err(format!("Failed to download assets manifest: {}", e)),
            };
        }
    }

    Err(format!("Failed to get parse version manifest"))
}

pub async fn is_asset_downloaded(manifest: &serde_json::Value, k: &String, v: &serde_json::Value) -> bool {
    if let Some(_hash) = v.get("hash") {
        if let Some(assets) = manifest["assets"].as_array() {
            for asset in assets {
                if let Some(asset_name) = asset["name"].as_str() {
                    println!("asset_name: {:#?} | k: {}, | v: {}", asset_name, k, v);
                } else {
                    println!("Asset not found");
                }
            }
        } else {
            println!("Assets array not found");
        }
    }

    true
}

pub fn is_array_exists(manifest: &serde_json::Value, array: &str) -> bool {
    if let Some(_) = manifest[array].as_array() {
        true
    } else {
        false
    }
}