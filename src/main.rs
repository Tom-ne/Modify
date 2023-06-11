pub mod commands;
pub mod constants;
pub mod lib;

use crate::lib::{
    io::io_helper::clear,
    modify::command_handler::{create_command_handler, print_help_menu},
    virus_scan::virus_scanner::scan,
};
use tokio;

use crate::lib::io::io_helper::{flush_output_stream, get_user_input};

#[tokio::main] // Use the tokio runtime
async fn main() {
    clear();
    print_help_menu();

    let command_handler = create_command_handler();

    let mut input = String::new();

    while input != "q" {
        print!("Please enter your selection (enter h for help): ");
        flush_output_stream();
        input = get_user_input();

        if let Some(command) = command_handler.get(input.as_str()) {
            command.run().await;
        } else {
            println!("Invalid command!");
        }
        scan();
    }
}
