use std::fs;

use async_trait::async_trait;

use crate::lib::{
    io::io_helper::get_user_input,
    modify::{command::Command, config_helper::read_config},
};

pub struct UninstallCommand;

fn get_mods_in_directory(dir_path: &str) -> Vec<String> {
    let mut mods: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy().into_owned();
                mods.push(file_name_str);
            }
        }
    }

    mods
}

#[async_trait]
impl Command for UninstallCommand {
    async fn run(&self) {
        println!("What mod would you like to uninstall?");
        let input = get_user_input().to_string();

        let dir_path = read_config().unwrap().mc_mod_dir;
        let mods = get_mods_in_directory(&dir_path);

        for mc_mod in mods {
            if mc_mod.starts_with(&input) {
                let filepath = format!("{}{}", &dir_path, mc_mod);
                match fs::remove_file(&filepath) {
                    Ok(()) => println!("File removed successfully"),
                    Err(err) => println!("Failed to remove file: {}", err),
                }
            }
        }
    }

    fn description(&self) -> &str {
        "Uninstall a mod"
    }
}
