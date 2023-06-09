use async_trait::async_trait;

use crate::{constants::CONFIG_FILE_PATH, lib::mod_manager::{mod_manager_settings::ModManagerSettings, config_helper::read_config, command::Command}};

pub struct PrintConfigCommand;

#[async_trait]
impl Command for PrintConfigCommand {
    async fn run(&self) {
        let config_path = CONFIG_FILE_PATH;
        let settings: ModManagerSettings = read_config(config_path).unwrap();
    
        ModManagerSettings::print(settings);
    }
}
