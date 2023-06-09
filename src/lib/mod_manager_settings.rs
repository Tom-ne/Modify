use serde::{Deserialize, Serialize};

use crate::lib::io_helper::print_middle;

#[derive(Debug, Serialize, Deserialize)]
pub enum Loader {
    Fabric,
    Forge,
    Quilt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCData {
    pub version: String,
    pub mod_loader: Loader,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModManagerSettings {
    pub mc_mod_dir: String,
    pub minecraft_data: MCData,
    pub multi_mc_dir: String,
}

impl ModManagerSettings {
    pub fn new(mc_mod_dir: String, minecraft_data: MCData, multi_mc_dir: String) -> Self {
        ModManagerSettings {
            mc_mod_dir,
            minecraft_data,
            multi_mc_dir,
        }
    }

    pub fn print(settings: ModManagerSettings) {
        let separator = "==============================================";
        let title = "Mod Manager config";
        print_middle(separator, title);
        println!("• Minecraft Mods directory: {}", settings.mc_mod_dir);
        println!("• Minecraft Version: {}", settings.minecraft_data.version);
        println!("• Mod Loader: {:?}", settings.minecraft_data.mod_loader);
        println!("• Multi MC directory: {:?}", settings.multi_mc_dir);
    }
}

