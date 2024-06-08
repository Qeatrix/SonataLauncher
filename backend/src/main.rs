use std::fs;
use async_std::fs::create_dir;
use async_std::task::block_on;
use serde_json;
use tide::Request;
use tide::prelude::*;
use surf;

pub mod instance;
use instance::Instance;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.at("/init/root").post(init_sonata_root);
    app.at("/instance/download_versions").get(get_versions);
    app.at("/instance/create").post(create_instance);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}


async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    let response_body = json!({
        "message": format!("Hello, {}! I've put in an order for {} shoes", name, legs)
    });

    Ok(tide::Response::builder(200)
        .body(response_body)
        .content_type(tide::http::mime::JSON)
        .build())
}

#[derive(Deserialize)]
struct LauncherRoot {
    path: String,
}

async fn init_sonata_root(mut req: Request<()>) -> tide::Result {
    let LauncherRoot { path } = req.body_json().await?;
    println!("Trying to create new root: {path}");

    let response: serde_json::Value;
    match block_on(create_dir(&path)) {
        Ok(_) => {
            println!("Created new root: {:?}", fs::canonicalize(path));
            response = json!({
                "result": "Launcher root successfuly created"
            });
        },
        Err(e) => {
            println!("Failed to create root: {:?}", fs::canonicalize(path));
            response = json!({
                "result": format!("Failed to initialize launcher root: {}", e)
            });
        },
    }

    Ok(tide::Response::builder(200)
        .body(response)
        .content_type(tide::http::mime::JSON)
        .build())
}

async fn get_versions(_req: Request<()>) -> tide::Result {
    let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
    match surf::get(url).await {
        Ok(mut response) => {
            match response.body_json::<serde_json::Value>().await {
                Ok(data) => {
                    Ok(tide::Response::builder(200)
                        .body(data)
                        .content_type(tide::http::mime::JSON)
                        .build())
                },
                Err(_) => {
                    Ok(tide::Response::builder(500)
                        .body(json!({ "message": "Failed to parse JSON" }))
                        .content_type(tide::http::mime::JSON)
                        .build())
                }
            }
        },

        Err(_) => {
            Ok(tide::Response::builder(500)
                .body(json!({ "message": "Failed to download versions manifest" }))
                .content_type(tide::http::mime::JSON)
                .build())
        }
    }
}


#[derive(Deserialize)]
struct InstanceRequest {
    name: String,
    version: String
}

async fn create_instance(mut req: Request<()>) -> tide::Result {
    let InstanceRequest { name, version } = req.body_json().await?;

    let response: serde_json::Value;
    match Instance::init(name, version) {
        Ok(result) => {
            response = json!({
                "result": format!("Created, {}", result)
            });

        },

        Err(e) => {
            response = json!({
                "result": format!("Failed to create instance, {}", e)
            });
        }
    }

    Ok(tide::Response::builder(200)
        .body(response)
        .content_type(tide::http::mime::JSON)
        .build())
}