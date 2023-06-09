use async_trait::async_trait;

use crate::{
    lib::mod_manager::{
        command::Command, config_helper::read_config, mod_manager_settings::ModManagerSettings,
    },
};

pub struct PrintConfigCommand;

#[async_trait]
impl Command for PrintConfigCommand {
    async fn run(&self) {
        let settings: ModManagerSettings = read_config().unwrap();

        ModManagerSettings::print(settings);
    }

    fn description(&self) -> &str {
        "print current Modify configuration"
    }
}
