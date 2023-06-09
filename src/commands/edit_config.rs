use crate::lib::{config_helper::{read_config, write_config}};

pub(crate) async fn run() {
    let config_path = "config.json";
    let mut settings = read_config(config_path).unwrap();

    settings.mc_mod_dir = String::from("new_mod_dir");
    settings.minecraft_data.version = String::from("1.19.3");

    write_config(config_path, &settings).unwrap();

    // println!("{:#?}", settings);
}