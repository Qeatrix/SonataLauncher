use std::io::{Error, ErrorKind};
use std::fs::create_dir;
use tide::prelude::Deserialize;

#[derive(Deserialize)]
pub struct LauncherRoot {
    pub path: String,
}

impl LauncherRoot {
    pub fn init_root(&self) -> String {
        println!("path: {}", self.path);

        match Self::init_root_folder(&self.path) {
            Ok(_) => {},
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => {
                    println!("The root directory already exists, continuing...");
                },

                _ => {
                    return format!("Failed to initialize root directory: {}", e)
                }
            }
        }
    
        let libs_path = format!("{}{}", self.path, "/libraries");
        match Self::init_libs_folder(&libs_path) {
            Ok(_) => {},
            Err(e) => return format!("Failed to initialize libraries dir: {}", e)
        }

        format!("Root directory initialized successfully")
    }

    fn init_root_folder(path: &String) -> Result<(), Error> {
        match create_dir(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }


    fn init_libs_folder(path: &String) -> Result<(), String> {
        match create_dir(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }
}