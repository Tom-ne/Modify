use async_trait::async_trait;
use std::convert::TryInto;

use crate::{
    constants::CONFIG_FILE_PATH,
    lib::{
        io::io_helper::{flush_output_stream, get_user_input, print_middle},
        modify::{
            command::Command,
            config_helper::{read_config, write_config},
            modify_settings::ModLoader,
        },
        modrinth::get_mc_version::get_mc_versions,
    },
};

pub struct EditConfigCommand;

fn print_menu() {
    let separator = "==============================================";
    let title = "Modify config Editor";
    print_middle(separator, title);
    println!("• mdir - Mods Directory Path");
    println!("• mc_version - Minecraft Version");
    println!("• mloader - Mod Loader");
    println!("• multimc - Multi MC Directory");
}

#[async_trait]
impl Command for EditConfigCommand {
    async fn run(&self) {
        let config_path = CONFIG_FILE_PATH;
        let mut settings = read_config().unwrap();

        print_menu();
        println!("What would you like to edit?");
        let input = get_user_input();

        match input.as_str() {
            "mdir" => {
                print!("Enter new Mods directory path: ");
                flush_output_stream();
                let input = get_user_input();
                settings.mc_mod_dir = String::from(input);
            }
            "mc_version" => {
                print!("Enter new Minecraft version: ");
                flush_output_stream();
                let input = get_user_input();

                let mc_versions = get_mc_versions().await.unwrap();
                println!("{:?}", mc_versions);

                if !mc_versions.contains(&input) {
                    println!("Invalid Minecraft version!");
                    return;
                }
                settings.minecraft_data.version = input;
            }
            "mloader" => {
                println!("Enter new Mod loader:");
                println!("1. Fabric");
                println!("2. Forge");
                println!("3. Quilt");
                flush_output_stream();
                let loader_option: i32 = get_user_input().parse().unwrap_or(0);
                match ModLoader::from_number((loader_option - 1).try_into().unwrap()) {
                    Some(mod_loader) => {
                        settings.minecraft_data.mod_loader = mod_loader;
                    }
                    None => println!("Invalid Mod Loader option!"),
                }
            }
            "multimc" => {
                print!("Enter new Multi MC directory path: ");
                flush_output_stream();
                let input = get_user_input();
                settings.multi_mc_dir = String::from(input);
            }
            _ => {
                println!("Invalid option!");
                return;
            }
        }

        write_config(config_path, &settings).unwrap();

        println!("Configuration updated successfully!");
        settings.print();
    }

    fn description(&self) -> &str {
        "Configure Modify"
    }
}
