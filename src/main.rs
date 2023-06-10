use lib::{
    io::io_helper::clear,
    modify::command_handler::{create_command_handler, print_help_menu},
};
use tokio;

use crate::lib::io::io_helper::{flush_output_stream, get_user_input};

mod constants;

mod lib {
    pub mod io {
        pub mod io_helper;
    }

    pub mod modify {
        pub mod command;
        pub mod command_handler;
        pub mod config_helper;
        pub mod modify_settings;
    }

    pub mod modrinth {
        pub mod get_project;
        pub mod get_versions;
        pub mod request_handler;
        pub mod search_req;
    }
}

mod commands {
    pub mod configuration {
        pub mod edit_config;
        pub mod print_config;
    }
    pub mod cmd {
        pub mod help;
        pub mod install;
        pub mod list_mods;
        pub mod quit;
        pub mod search;
        pub mod uninstall;
    }
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
