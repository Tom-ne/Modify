use std::collections::HashMap;
use crate::commands::{search::SearchCommand, install::InstallCommand, print_config::PrintConfigCommand, edit_config::EditConfigCommand};

use super::command::Command;

pub(crate) fn create_command_handler() -> HashMap<&'static str, Box<dyn Command>> {
    let mut dispatcher: HashMap<&'static str, Box<dyn Command>> = HashMap::new();
    dispatcher.insert("sS", Box::new(SearchCommand));
    dispatcher.insert("S", Box::new(InstallCommand));
    dispatcher.insert("pconfig", Box::new(PrintConfigCommand));
    dispatcher.insert("config", Box::new(EditConfigCommand));
    // Add more commands here

    dispatcher
}
