use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub name: String,
    pub image_url: String,
    pub auto_export_path: String,
    pub auto_export: bool,
    pub config_folder: String,
    pub export_folder: String,
    pub engine_interval: u32,
    pub auto_refresh: u32,
    pub notif_icon: String,
    pub notif_icon_path: String,
    pub competition_url: String,
    pub competition_mode: bool,
}

impl Config {
    pub fn get(conf: String) -> Self {
        let mut file = File::open(format!("{}/config.json", conf)).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}
