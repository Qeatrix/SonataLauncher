use std::collections::HashMap;
use home::home_dir;

use async_std::stream::StreamExt;
use serde_json;
use tide::Request;
use tide::prelude::*;
use surf;

pub mod instance;
use instance::Instance;

pub mod java;
use java::Java;

pub mod root;
use root::LauncherRoot;

use tide_websockets::Message;
use tide_websockets::WebSocket;
use tide_websockets::WebSocketConnection;

mod threadpool;

mod types;
mod utils;


#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // let available_java_url = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";
    match home_dir() {
        Some(path) => {
            let path = format!("{}/.sonata/java", path.display());
            let java_properties = Java::new("21".to_string(), "java-runtime-delta".to_string(), path);
            Java::init(java_properties).await;
        },
        None => (),
    };

    let mut app = tide::new();

    // Example Data
    app.at("/orders/shoes").post(order_shoes);

    // Init routes
    app.at("/init/root").post(handle_init_root);

    // Java routes
    app.at("/ws/java/install").get(WebSocket::new(|_req, ws| download_java_ws(ws)));

    // Instance routes
    app.at("/instance/download_versions").get(get_versions);
    // app.at("/instance/create").post(create_instance);
    app.at("/ws/instance/create").get(WebSocket::new(|_req, ws| create_instance_ws(ws)));

    app.at("/debug/ws").get(WebSocket::new(|_req, stream| debug_ws(stream)));

    // Run server
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}



#[derive(Debug, Deserialize)]
struct DownloadRequest {
    java_ver: String,
}

async fn download_java_ws(mut ws: WebSocketConnection) -> tide::Result<()> {
    while let Some(Ok(Message::Text(input))) = ws.next().await {
        let download_request: DownloadRequest = serde_json::from_str(&input).map_err(|e| {
            tide::Error::from_str(400, format!("Failed to parse recieved JSON: {}", e))
        })?;

        let DownloadRequest { java_ver } = download_request;
        // let available_java_url = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";

        match home_dir() {
            Some(path) => {
                let path = format!("{}/.sonata", path.display());
                let java_properties = Java::new("21".to_string(), "java-runtime-delta".to_string(), path);
                Java::init(java_properties).await;
            },
            None => (),
        };

        println!("Recieved java version: {}", java_ver);
    }

    Ok(())
}

async fn create_instance_ws(mut ws: WebSocketConnection) -> tide::Result<()> {
    while let Some(Ok(Message::Text(input))) = ws.next().await {
        let instance_request: InstanceRequest = serde_json::from_str(&input).map_err(|e| {
            tide::Error::from_str(400, format!("Failed to parse recieved JSON: {}", e))
        })?;

        let InstanceRequest { name, url, info } = instance_request;

        for (k, v) in info.iter() {
            println!("k: {}, v: {}", k, v);
        }

        let response: serde_json::Value;
        match Instance::init(&mut Instance::new(name, url, info), &ws).await {
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

        ws.send_string(format!("{response}")).await?;
    }

    Ok(())
}

async fn debug_ws(mut stream: WebSocketConnection) -> tide::Result<()> {
    while let Some(Ok(Message::Text(input))) = stream.next().await {
        let output: String = input.chars().rev().collect();

        for _ in 0..10 {
            stream
                .send_string(format!("{} | {}", &input, &output)).await?;
        }
    }

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


async fn handle_init_root(mut req: Request<()>) -> tide::Result {
    let launcher_root: LauncherRoot = req.body_json().await?;

    let response = json!({ "message": launcher_root.init_root() });

    Ok(tide::Response::builder(200)
        .body(response)
        .content_type(tide::http::mime::JSON)
        .build())
}


async fn get_versions(_req: Request<()>) -> tide::Result {
    let url = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

    let result;
    let code;

    match surf::get(url).await {
        Ok(mut response) => {
            match response.body_json::<serde_json::Value>().await {
                Ok(data) => {
                    result = data;
                    code = 200;
                },
                Err(_) => {
                        result = json!({ "message": "Failed to parse JSON" });
                        code = 500;
                }
            }
        },

        Err(_) => {
                result = json!({ "message": "Failed to download versions manifest" });
                code = 500;
        }
    }

    Ok(tide::Response::builder(code)
        .body(result)
        .content_type(tide::http::mime::JSON)
        .build())
}


#[derive(Debug, Deserialize)]
struct InstanceRequest {
    name: String,
    url: String,
    info: HashMap<String, String>
}

// async fn create_instance(mut req: Request<()>) -> tide::Result {
//     let InstanceRequest { name, url, info } = req.body_json().await?;

//     for (k, v) in info.iter() {
//         println!("k: {}, v: {}", k, v);
//     }

//     let response: serde_json::Value;
//     match Instance::init(&mut Instance::new(name, url, info)).await {
//         Ok(result) => {
//             response = json!({
//                 "result": format!("Created, {}", result)
//             });

//         },

//         Err(e) => {
//             response = json!({
//                 "result": format!("Failed to create instance, {}", e)
//             });
//         }
//     }

//     Ok(tide::Response::builder(200)
//         .body(response)
//         .content_type(tide::http::mime::JSON)
//         .build())
// }
