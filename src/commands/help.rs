use async_trait::async_trait;

use crate::lib::mod_manager::{command::Command, command_handler::print_help_menu};

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    async fn run(&self) {
        print_help_menu();
    }

    fn description(&self) -> &str {
        "Prints the help menu"
    }
}
