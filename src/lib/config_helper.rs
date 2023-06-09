use std::{
    fs::File,
    io::{self, Read, Write},
};

use super::mod_manager_settings::ModManagerSettings;

pub(crate) fn read_config(path: &str) -> io::Result<ModManagerSettings> {
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
