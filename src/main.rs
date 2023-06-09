use lib::io_helper::get_user_input;
use tokio;

use crate::lib::io_helper::{flush_output_stream, clear};

mod lib {
    pub mod io_helper;
    pub mod config_helper;
    pub mod mod_manager_settings;
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

    let mut input = String::new();

    while input != "q" {
        print!("Please enter your selection: ");
        flush_output_stream();
        input = get_user_input();
        if input == "sS" {
            commands::search::run().await;
        } else if input == "S" {
            commands::install::run().await;
        } else if input == "pconfig" {
            commands::print_config::run().await;
        } else if input == "config" {
            commands::edit_config::run().await;
        } else if input == "q" {
            
        } else {
            println!("Invalid command!");
        }
    }
}