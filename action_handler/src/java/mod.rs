use std::{fs, os::unix::fs::PermissionsExt};

use async_std::process::Command;

pub mod arch;
pub mod install;

const JAVA_VERSIONS_MANIFEST_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";


pub struct Java {
    version: String,
    runtime_name: String,
    manifest_url: String,
    destination: String,
    sha1: String,
}

impl Java {
    pub fn new(version: String, runtime_name: String, destination: String) -> Java {
        Java {
            version,
            runtime_name,
            manifest_url: JAVA_VERSIONS_MANIFEST_URL.to_string(),
            destination,
            sha1: String::from(""),
        }
    }

    pub async fn init(mut self, metacache_path: String) {
        let exec_path = install::init(&mut self, metacache_path).await.unwrap();

        let metadata = fs::metadata(&exec_path).unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o775);
        fs::set_permissions(&exec_path, permissions).unwrap();

        let output = Command::new(exec_path)
            .arg("-version")
            .output()
            .await.unwrap();

        println!("{:#?}", output);
    }
}
