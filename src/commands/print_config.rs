use async_trait::async_trait;

use crate::lib::modify::{
    command::Command, config_helper::read_config, modify_settings::ModManagerSettings,
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
