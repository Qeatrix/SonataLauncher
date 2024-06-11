use serde_json;
use tide::Request;
use tide::prelude::*;
use surf;

pub mod instance;
use instance::Instance;

pub mod root;
use root::LauncherRoot;

mod threadpool;


#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    
    // Example Data
    app.at("/orders/shoes").post(order_shoes);

    // Init routes
    app.at("/init/root").post(handle_init_root);

    // Instance routes
    app.at("/instance/download_versions").get(get_versions);
    app.at("/instance/create").post(create_instance);

    // Run server
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
    version: String,
    url: String,
}

async fn create_instance(mut req: Request<()>) -> tide::Result {
    let InstanceRequest { name, version, url } = req.body_json().await?;

    let response: serde_json::Value;
    match Instance::init(&Instance::new(name, version, url)).await {
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