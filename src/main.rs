use std::{fs, io::{self, stdin, stdout, Write}, path::Path, {thread, time}};

use directories::BaseDirs;
use reqwest::{Error};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let version;

    /*
        I should probably use registry instead of making a request,
        this was my first thought on how I was gonna do it.
    */

    let client_settings = reqwest::get("https://clientsettingscdn.roblox.com/v1/client-version/WindowsPlayer")
    .await?
    .json::<Value>()
    .await?;

    if let Some(client_version_upload) = client_settings.get("clientVersionUpload") {
        version = client_version_upload.as_str().unwrap().to_string();
    } else {
        println!("> roblox client version not found, lets try using another api instead");

        let new_req = reqwest::get("https://setup.rbxcdn.com/version")
        .await?
        .text()
        .await?;

        version = new_req;
    };

    if let Some(proj_dirs) = BaseDirs::new() {
        let local_appdata = proj_dirs.cache_dir();
        let directory = local_appdata.join("Roblox");
        let roblox_path = Path::new(&directory);

        if !roblox_path.is_dir() {
            println!("> roblox installation not found, try reinstalling roblox if already installed");

            thread::sleep(time::Duration::from_secs(2));

            return Ok(());
        }

        let versions = roblox_path.join("Versions");

        if !versions.is_dir() {
            println!("> versions doesn't exist, try reinstalling roblox");

            thread::sleep(time::Duration::from_secs(2));

            return Ok(());
        }

        let current_version = versions.join(version);

        if !current_version.is_dir() {

            println!("> unable to find roblox version, please select one instead\n");

            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                println!("> selected path {}\n", path.display());

                apply_fps_flag(&path).unwrap();

                return Ok(());
            } else {
                println!("> no path was selected");

                thread::sleep(time::Duration::from_secs(2));
                
                return Ok(());
            }
        }

        apply_fps_flag(&current_version).unwrap();
    }

    Ok(())
}

fn apply_fps_flag(path: &Path) -> io::Result<()> {
    let mut max_fps = String::new();

    let client_settings = path.join("ClientSettings");

    fs::create_dir_all(&client_settings)?;

    let mut client_app_settings = fs::File::create(&client_settings.join("ClientAppSettings.json"))?;

    print!("> select your maximum fps: ");

    stdout().flush()?;

    stdin().read_line(&mut max_fps)?;
    
    let flag = format!("{{\n  \"DFIntTaskSchedulerTargetFps\": {} }}", max_fps);

    client_app_settings.write_all(flag.as_bytes())?;
    
    println!("\n> fps unlocker successfully unlocked your fps!");

    thread::sleep(time::Duration::from_secs(2));
    
    Ok(())
}