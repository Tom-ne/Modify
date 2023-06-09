use lib::io::io_helper::clear;
use lib::mod_manager::command_handler::print_help_menu;
use tokio;

use crate::lib::io::io_helper::{flush_output_stream, get_user_input};
use crate::lib::mod_manager::command_handler::create_command_handler;

mod constants;

mod lib {
    pub mod io {
        pub mod io_helper;
    }

    pub mod mod_manager {
        pub mod command;
        pub mod command_handler;
        pub mod config_helper;
        pub mod mod_manager_settings;
    }

    pub mod modrinth {
        pub mod get_project;
        pub mod get_versions;
        pub mod request_handler;
        pub mod search_req;
    }
}

mod commands {
    pub mod edit_config;
    pub mod help;
    pub mod install;
    pub mod list_mods;
    pub mod print_config;
    pub mod quit;
    pub mod search;
}

#[tokio::main] // Use the tokio runtime
async fn main() {
    clear();
    print_help_menu();

    let command_handler = create_command_handler();

    let mut input = String::new();

    while input != "q" {
        print!("Please enter your selection: ");
        flush_output_stream();
        input = get_user_input();

        if let Some(command) = command_handler.get(input.as_str()) {
            command.run().await;
        } else {
            println!("Invalid command!");
        }
    }
}
