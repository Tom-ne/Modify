use std::fs;

use async_trait::async_trait;

use crate::lib::modify::{command::Command, config_helper::read_config};

pub struct ListCommand;

#[async_trait]
impl Command for ListCommand {
    async fn run(&self) {
        let dir_path = read_config().unwrap().mc_mod_dir;

        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    if let Some(index) = file_name_str.find('-') {
                        let part_before_number = &file_name_str[..index];
                        println!("{}", part_before_number);
                    }
                }
            }
        } else {
            println!("Failed to read directory");
        }
    }

    fn description(&self) -> &str {
        "list all mods"
    }
}
