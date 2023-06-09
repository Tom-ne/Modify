use crate::lib::{config_helper::{read_config}, mod_manager_settings::ModManagerSettings};

pub(crate) async fn run() {
    let config_path = "config.json";
    let settings: ModManagerSettings = read_config(config_path).unwrap();

    ModManagerSettings::print(settings);
}