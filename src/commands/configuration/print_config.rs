use async_trait::async_trait;

use crate::lib::modify::{
    command::Command, config_helper::read_config,
};

pub struct PrintConfigCommand;

#[async_trait]
impl Command for PrintConfigCommand {
    async fn run(&self) {
        read_config().unwrap().print();
    }

    fn description(&self) -> &str {
        "print current Modify configuration"
    }
}
