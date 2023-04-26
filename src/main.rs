#![windows_subsystem = "windows"]

use curl::easy::Easy;
use directories::{BaseDirs};
use std::io::{Write};
use std::path::Path;
use std::{fs, process};

use eframe::egui;
use rfd;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        resizable: false,
        decorated: true,
        follow_system_theme: true,
        max_window_size: Some(egui::vec2(300.0, 70.0)),
        ..Default::default()
    };

    eframe::run_native(
        "fps unlocker",
        options,
        Box::new(|_cc| Box::<Unlocker>::default()),
    )
}

#[derive(Default)]
struct Unlocker {
    picked_path: Option<String>,
}

impl eframe::App for Unlocker {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {

            let button = ui.add_sized([285., 25.], egui::Button::new("Install"));
            let manual = ui.add_sized([285., 25.], egui::Button::new("Manual Install"));

            if button.clicked() {
                let mut easy = Easy::new();
                easy.url("https://raw.githubusercontent.com/ScreamBirb/fps_unlocker/master/flags.json").unwrap();
                
                let version_request = minreq::get("http://setup.roblox.com/version.txt").send();
                let binding = version_request.expect("failed to send request");
                let version = binding.as_str().expect("failed to get version");

                if let Some(proj_dirs) = BaseDirs::new() {
                    let local_appdata = proj_dirs.cache_dir();
                    let directory = local_appdata.join("Roblox");
                    let roblox_path = Path::new(&directory);
                    if roblox_path.is_dir() {
                        if roblox_path.join("Versions").is_dir() {
                            let current_version = roblox_path.join("Versions").join(version);
                            if current_version.is_dir() {
                                let client_settings = current_version.join("ClientSettings");
                                if client_settings.is_dir() {
                                    let mut client_app_settings = fs::File::create(current_version.join("ClientSettings").join("ClientAppSettings.json")).unwrap();

                                    easy.write_function(move |data| {
                                        client_app_settings.write_all(data).unwrap();
                                        Ok(data.len())
                                    }).unwrap();

                                    easy.perform().unwrap();

                                    process::exit(0x0100)

                                } else {
                                    fs::create_dir_all(client_settings).unwrap();
                                    
                                    let mut client_app_settings = fs::File::create(current_version.join("ClientSettings").join("ClientAppSettings.json")).unwrap();

                                    easy.write_function(move |data| {
                                        client_app_settings.write_all(data).unwrap();
                                        Ok(data.len())
                                    }).unwrap();

                                    easy.perform().unwrap();

                                    process::exit(0x0100)
                                }
                            }
                        }
                    }
                }
            }

            if manual.clicked() {
                let mut easy = Easy::new();
                easy.url("https://raw.githubusercontent.com/ScreamBirb/fps_unlocker/master/flags.json").unwrap();
                if let Some(path) = rfd::FileDialog::new().pick_folder() {

                    self.picked_path = Some(path.display().to_string());

                    // create mew folder
                    fs::create_dir_all(path.join("ClientSettings")).unwrap();

                    // add client app settings
                    let mut client_app_settings = fs::File::create(path.join("ClientSettings").join("ClientAppSettings.json")).unwrap();
                    easy.write_function(move |data| {
                        client_app_settings.write_all(data).unwrap();
                        Ok(data.len())
                    }).unwrap();
                    easy.perform().unwrap();
                    
                    process::exit(0x0100)
                }
            }
        });
    }
}