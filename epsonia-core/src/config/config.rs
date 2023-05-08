#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    name: String,
    image_url: String,
    auto_export_path: String,
    auto_export: bool,
    config_folder: String,
    export_folder: String,
    engine_interval: u32,
    auto_refresh: u32,
    notif_icon: String,
    competition_mode: String,
    competition_mode: bool,
}

impl Config {
    pub fn get(conf: String) -> Self {
        let mut file = File::open(conf).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}
