struct Config {
    versions_manifest_url: String,
}

impl Config {
    pub fn process_config() {
        match std::env::var("VERSIONS_MANIFEST_URL") {
            Ok(val) => {
                println!("{val}");
            }
            Err(e) => {
                println!("{e}");
            }
        };
    }
}