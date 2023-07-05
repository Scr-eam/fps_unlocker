use std::{path::Path, fs};
use directories::BaseDirs;
use reqwest::Error;
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut max_fps = String::new();

    let version = reqwest::get("https://setup.rbxcdn.com/version")
    .await?
    .text()
    .await?;

    if let Some(proj_dirs) = BaseDirs::new() {
        let local_appdata = proj_dirs.cache_dir();
        let directory = local_appdata.join("Roblox");
        let roblox_path = Path::new(&directory);

        if !roblox_path.is_dir() {
            println!("roblox installation not found");
        }

        let versions = roblox_path.join("Versions");

        if !versions.is_dir() {
            println!("versions doesn't exist, try reinstalling your roblox");
        }

        let current_version = versions.join(version.as_str());

        if current_version.is_dir() {
            fs::create_dir_all(current_version.join("ClientSettings")).unwrap();
            
            let mut client_app_settings = fs::File::create(current_version.join("ClientSettings").join("ClientAppSettings.json")).unwrap();

            print!("> select your maxium fps: ");

            stdout().flush().unwrap();

            stdin().read_line(&mut max_fps).expect("failed to read input");

            let flag = "{\n  \"DFIntTaskSchedulerTargetFps\": ".to_owned() +  &max_fps + "}";

            client_app_settings.write_all(flag.as_bytes()).unwrap();

            println!("\n> fps unlocker successfully unlocked your fps!");

            std::process::abort();
        }

        println!("> current roblox version not found!\n");

        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            println!("> selected path {}\n", path.display());

            fs::create_dir_all(path.join("ClientSettings")).unwrap();
            
            let mut client_app_settings = fs::File::create(path.join("ClientSettings").join("ClientAppSettings.json")).unwrap();

            print!("> select your maxium fps: ");

            stdout().flush().unwrap();

            stdin().read_line(&mut max_fps).expect("failed to read input");

            let flag = "{\n  \"DFIntTaskSchedulerTargetFps\": ".to_owned() +  &max_fps + "}";

            client_app_settings.write_all(flag.as_bytes()).unwrap();

            println!("\n> fps unlocker successfully unlocked your fps!");

            std::process::abort();
        }
    }

    loop {}
}