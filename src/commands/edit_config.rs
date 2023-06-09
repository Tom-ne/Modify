use crate::lib::{
    config_helper::{read_config, write_config},
    mod_manager_settings::{ModManagerSettings, ModLoader},
    io_helper::{print_middle, get_user_input, flush_output_stream},
};

fn print_menu() {
    let separator = "==============================================";
    let title = "Mod Manager config Editor";
    print_middle(separator, title);
    println!("• mdir - Mods directory path");
    println!("• mc_version - Minecraft version");
    println!("• mloader - Mod Loader");
    println!("• multimc - Multi MC Directory");
}

pub(crate) async fn run() {
    let config_path = "config.json";
    let mut settings = read_config(config_path).unwrap();

    print_menu();
    println!("What would you like to edit?");
    let input = get_user_input();

    if input == "mdir" {
        print!("Enter new Mods directory path: ");
        flush_output_stream();
        let input = get_user_input();
        settings.mc_mod_dir = String::from(input);
    } else if input == "mc_version" {
        print!("Enter new Minecraft version: ");
        flush_output_stream();
        let input = get_user_input();
        settings.minecraft_data.version = input;
    } else if input == "mloader" {
        print!("Enter new Mod loader (1: Fabric, 2: Forge, 3: Quilt): ");
        flush_output_stream();
        let loader_option: i32 = get_user_input().parse().unwrap();
        settings.minecraft_data.mod_loader = ModLoader::from_number((loader_option - 1).try_into().unwrap()).unwrap();
    } else if input == "multimc" {
        print!("Enter new multimc directory path: ");
        flush_output_stream();
        let input = get_user_input();
        settings.multi_mc_dir = String::from(input);
    } else {
        println!("Invalid option!");
    }

    write_config(config_path, &settings).unwrap();

    ModManagerSettings::print(settings);
}
