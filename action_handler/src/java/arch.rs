use std::env::consts::{ARCH, OS};

const UNSUPPORTED_ARCH_MSG: &str = "Your system architecture is unsupported";
const UNSUPPORTED_OS_MSG: &str = "Your system is unsupported";

struct ManifestArches;

impl ManifestArches {
    pub const LINUX: &'static str = "linux";
    pub const LINUX_X32: &'static str = "linux-i386";
    pub const MACOS: &'static str = "mac-os";
    pub const MACOS_ARM64: &'static str = "mac-os-arm64";
    pub const WINDOWS_X32: &'static str = "windows-x32";
    pub const WINDOWS_X64: &'static str = "windows-x64";
    pub const WINDOWS_ARM64: &'static str = "windows-arm64";
}

pub fn manifest_osarch<'a>() -> Option<&'a str> {
    let os = OS;

    if os == "linux" {
        match ARCH {
            "x86" => {
                return Some(ManifestArches::LINUX_X32);
            },
            "x86_64" => {
                return Some(ManifestArches::LINUX);
            },
            _ => {
                println!("{}", UNSUPPORTED_ARCH_MSG);
                return None;
            }
        }
    } else if os == "macos" {
        match ARCH {
            "x86" => {
                return Some(ManifestArches::MACOS);
            },
            "x86_64" => {
                return Some(ManifestArches::MACOS);
            },
            "aarch64" => {
                return Some(ManifestArches::MACOS_ARM64);
            }
            _ => {
                println!("{}", UNSUPPORTED_ARCH_MSG);
                return None;
            }
        }
    } else if os == "windows" {
        match ARCH {
            "x86" => {
                return Some(ManifestArches::WINDOWS_X32);
            }
            "x86_64" => {
                return Some(ManifestArches::WINDOWS_X64);
            },
            "aarch64" => {
                return Some(ManifestArches::WINDOWS_ARM64);
            }
            _ => {
                println!("{}", UNSUPPORTED_ARCH_MSG);
                return None;
            }
        }
    } else {
        println!("{}", UNSUPPORTED_OS_MSG);
        return None;
    }
}
