use serde::{Deserialize, Serialize};

use crate::lib::io::io_helper::print_middle;

#[derive(Debug, Serialize, Deserialize)]
pub enum ModLoader {
    Fabric,
    Forge,
    Quilt,
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MCData {
    pub version: String,
    pub mod_loader: ModLoader,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModManagerSettings {
    pub mc_mod_dir: String,
    pub minecraft_data: MCData,
    pub multi_mc_dir: String,
}

impl ModLoader {
    pub fn from_number(number: u32) -> Option<ModLoader> {
        match number {
            0 => Some(ModLoader::Fabric),
            1 => Some(ModLoader::Forge),
            2 => Some(ModLoader::Quilt),
            _ => Some(ModLoader::None),
        }
    }
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
        let title = "Modify config";
        print_middle(separator, title);
        println!("• Minecraft Mods directory: {}", settings.mc_mod_dir);
        println!("• Minecraft Version: {}", settings.minecraft_data.version);
        println!("• Mod Loader: {:?}", settings.minecraft_data.mod_loader);
        println!("• Multi MC directory: {:?}", settings.multi_mc_dir);
    }
}
