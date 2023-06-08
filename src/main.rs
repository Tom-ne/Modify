mod api;
mod input_helper;
use input_helper::{get_user_input, flush_output_stream};
use tokio;

mod commands {
    pub mod search;
}

fn print_menu() {
    println!("==============================================");
    println!("\t\tMod Manager {}", env!("CARGO_PKG_VERSION"));
    println!("==============================================");
    println!("• v - set Minecraft version");
    println!("• Syu - update all mods");
    println!("• S - install mods");
    println!("• sS - search for mods");
    println!("• l - list all mods");
    println!("• h - prints this menu");
    println!("• q - quits the program");
    print!("Please enter your selection: ");
    flush_output_stream();
}

#[tokio::main] // Use the tokio runtime
async fn main() {
    print_menu();

    let input = get_user_input();

    if input == "q" {
        println!("Exiting...");
    } else if input == "sS" {
        commands::search::run().await;
    }
}
