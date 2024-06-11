use std::fmt::format;

use async_std::process::Command;

use crate::instance::LaunchArgs;

pub async fn launch_instance(manifest: serde_json::Value, mut args: LaunchArgs) {
    // extract_launch_args(manifest);
    let finished_args_array = define_launch_args(args).await;

    let mut finished_args = String::new();
    for arg in finished_args_array.iter() {
        finished_args.push_str(&format!("{} ", arg));
    }
    println!("{}", finished_args);

    let output = Command::new("/nix/store/2vwkssqpzykk37r996cafq7x63imf4sp-openjdk-21+35/bin/java").arg(finished_args).output().await.unwrap();
    println!("{:#?}", output);
}

async fn define_launch_args(mut args: LaunchArgs) -> Vec<String> {
    let mut tmp_args = Vec::new();

    // Java executable
    let java_path = "java".to_string();
    tmp_args.push(java_path);


    tmp_args.push("-cp".to_string());

    let mut solid_paths_str = String::new();
    for path in args.libs_dir.iter() {
        solid_paths_str.push_str(path);
    }
    tmp_args.push(solid_paths_str);

    tmp_args.push(args.main_class);

    tmp_args.push("--username".to_string());
    tmp_args.push(args.username);

    tmp_args.push("--version".to_string());
    tmp_args.push(args.version);

    tmp_args.push("--gameDir".to_string());
    tmp_args.push(args.game_dir);

    tmp_args.push("--assetsDir".to_string());
    tmp_args.push(args.assets_dir);

    tmp_args.push("--assetIndex".to_string());
    tmp_args.push(args.asset_index);

    tmp_args.push("--uuid".to_string());
    tmp_args.push(args.uuid);

    tmp_args.push("--accessToken".to_string());
    tmp_args.push(args.access_token);

    tmp_args.push("--clientId".to_string());
    tmp_args.push(args.client_id);

    tmp_args.push("xId".to_string());
    tmp_args.push(args.x_id);

    tmp_args.push("userType".to_string());
    tmp_args.push(args.user_type);

    tmp_args.push("-versionType".to_string());
    tmp_args.push(args.version_type);

    tmp_args
}

fn extract_launch_args<'a>(manifest: serde_json::Value) -> Vec<(&'a str, &'a str)> {
    if let Some(arguments) = manifest["arguments"]["game"].as_array() {
        for argument in arguments {
            println!("{}", argument);
        }
    }

    vec![("asd", "asd")]
}