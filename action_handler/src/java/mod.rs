pub mod arch;
pub mod install;

const JAVA_VERSIONS_MANIFEST_URL: &str = "https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json";


pub struct Java {
    version: String,
    runtime_name: String,
    manifest_url: String,
    destination: String,
}

impl Java {
    pub fn new(version: String, runtime_name: String, destination: String) -> Java {
        Java {
            version,
            runtime_name,
            manifest_url: JAVA_VERSIONS_MANIFEST_URL.to_string(),
            destination,
        }
    }

    pub async fn init(self) {
        install::init(&self).await.unwrap();
    }
}
