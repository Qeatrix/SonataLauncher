use std::collections::HashMap;

use async_std::fs::create_dir_all;

pub mod download;

pub struct Instance<'a> {
    name: String,
    version: String,
    url: String,
    libs: HashMap<&'a str, (&'a str, &'a str)> 
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
        match download::download_version_manifest(&self.url).await {
            Ok(_) => {},
            Err(e) => return Err(format!("Failed to download version manifest: {}", e))
        };

        match Self::init_instance_dir(&self.name).await {
            Ok(_) => println!("Instance directory created, continuing..."),
            Err(e) => return Err(format!("Failed to initialize instance directory: {}", e))
        };

        Ok(format!("asd"))
    }

    async fn init_instance_dir(name: &String) -> Result<(), String> {
        match create_dir_all(format!("/home/quartix/.sonata/instances/{}", name)).await {
            Ok(_) => {
                println!("Created instance dir");
                Ok(())
            },
            Err(e) => {
                return Err(format!("Failed to create instance dir: {}", e));
            }
        }
    }
}