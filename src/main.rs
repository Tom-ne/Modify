use lib::io::io_helper::clear;
use tokio;

use crate::lib::io::io_helper::{flush_output_stream, get_user_input};
use crate::lib::mod_manager::command_handler::create_command_handler;


mod constants;

mod lib {
    pub mod io {
        pub mod io_helper;
    }

    pub mod mod_manager {
        pub mod config_helper;
        pub mod mod_manager_settings;
        pub mod command;
        pub mod command_handler;
    }

    pub mod modrinth {
        pub mod request_handler;
        pub mod get_project;
        pub mod search_req;
        pub mod get_versions;
    }
}

mod commands {
    pub mod search;
    pub mod install;
    pub mod edit_config;
    pub mod print_config;
}


fn print_menu() {
    println!("==============================================");
    println!("\t\tMod Manager {}", env!("CARGO_PKG_VERSION"));
    println!("==============================================");
    println!("• pconfig - print current Mod Manager configuration");
    println!("• config - configure Mod Manager");
    println!("• v - set Minecraft version");
    println!("• Syu - update all mods");
    println!("• S - install mods");
    println!("• sS - search for mods");
    println!("• l - list all mods");
    println!("• h - prints this menu");
    println!("• q - quits the program");
}

#[tokio::main] // Use the tokio runtime
async fn main() {
    clear();
    print_menu();

    let command_handler = create_command_handler();

    let mut input = String::new();

    while input != "q" {
        print!("Please enter your selection: ");
        flush_output_stream();
        input = get_user_input();

        if let Some(command) = command_handler.get(input.as_str()) {
            command.run().await;
        } else if input == "q" {

        } else {
            println!("Invalid command!");
        }
    }
}