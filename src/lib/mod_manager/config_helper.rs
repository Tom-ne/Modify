use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use crate::constants::CONFIG_FILE_PATH;

use super::mod_manager_settings::{MCData, ModLoader, ModManagerSettings};

fn file_exists(path: &str) -> bool {
    let file_path = Path::new(path);
    file_path.exists()
}

// pub(crate) fn read_config(path: &str) -> io::Result<ModManagerSettings> {
pub(crate) fn read_config() -> io::Result<ModManagerSettings> {
    let path = CONFIG_FILE_PATH;
    if !file_exists(path) {
        write_config(
            path,
            &ModManagerSettings::new(
                "/path/to/mods".to_string(),
                MCData {
                    version: "1.20".to_string(),
                    mod_loader: ModLoader::Fabric,
                },
                "/path/to/multimc".to_string(),
            ),
        )?;
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let settings: ModManagerSettings = serde_json::from_str(&contents)?;
    Ok(settings)
}

pub(crate) fn write_config(path: &str, settings: &ModManagerSettings) -> io::Result<()> {
    let json = serde_json::to_string(settings)?;

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
