use serde::{Deserialize, Serialize};

use crate::lib::io::io_helper::print_middle;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

    pub fn from_string(input: String) -> ModLoader {

        match input.to_lowercase().as_str() {
            "fabric" => return ModLoader::Fabric,
            "forge" => return ModLoader::Forge,
            "quilt" => return ModLoader::Quilt,
            _ => return ModLoader::None
        };
    }

    pub fn from_list(input: Vec<String>) -> Vec<ModLoader> {
        input
            .into_iter()
            .map(|loader| ModLoader::from_string(loader.to_lowercase().to_string()))
            .collect()
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

    pub fn print(&self) {
        let separator = "==============================================";
        let title = "Modify config";
        print_middle(separator, title);
        println!("• Minecraft Mods directory: {}", self.mc_mod_dir);
        println!("• Minecraft Version: {}", self.minecraft_data.version);
        println!("• Mod Loader: {:?}", self.minecraft_data.mod_loader);
        println!("• Multi MC directory: {:?}", self.multi_mc_dir);
    }
}
