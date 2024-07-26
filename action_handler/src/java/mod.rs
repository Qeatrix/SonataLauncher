use std::{fs::{self, set_permissions}, os::unix::fs::PermissionsExt};

use async_std::process::Command;

pub mod arch;
pub mod extract;
pub mod download;

use crate::utils::metacache;

const JAVA_VERSIONS_MANIFEST_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";


#[derive(Eq, Hash, PartialEq, Debug)]
pub struct EntryInfo {
    path: String,
    path_type: String,
}

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

    pub async fn init(mut self, metacache_path: String) -> Result<(), String> {
        let mut metacache_value = match metacache::get_json(&metacache_path) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        let exec_path = extract::start_extraction(metacache_path, &mut metacache_value, &mut self).await.unwrap();

        if let Err(e) = Self::set_permissions(&exec_path) {
            return Err(e);
        }

        let output = Command::new(exec_path)
            .arg("--version")
            .output()
            .await.unwrap();

        println!("{:#?}", output);

        Ok(())
    }

    fn set_permissions(exec_path: &String) -> Result<(), String> {
        let metadata = match fs::metadata(&exec_path) {
            Ok(data) => data,
            Err(e) => return Err(e.to_string()),
        };

        let mut permissions = metadata.permissions();
        permissions.set_mode(0o775);

        match fs::set_permissions(&exec_path, permissions) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
