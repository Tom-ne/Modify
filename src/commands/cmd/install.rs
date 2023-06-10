use async_trait::async_trait;
use serde_json::Value;
use std::io::{self, Error, ErrorKind};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::lib::{
    io::io_helper::{flush_output_stream, get_user_input},
    modify::{command::Command, config_helper::read_config, modify_settings::ModLoader},
    modrinth::{
        get_project::get_project, get_versions::get_mod_versions, request_handler::make_request,
    },
};

pub struct InstallCommand;

fn get_dep_name(input: &str) -> Option<&str> {
    let mut started = false;
    let mut start_index = 0;

    for (idx, c) in input.char_indices() {
        if c.is_alphabetic() && !started {
            started = true;
            start_index = idx;
        } else if started && c.is_digit(10) {
            return Some(&input[start_index..idx - 1]);
        }
    }

    None
}

async fn install_dep(dep_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let req = format!("https://api.modrinth.com/v2/version/{}", dep_id);

    let json = make_request(req, String::new())
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Error: {:?}", err)))?;

    let name = get_dep_name(json["name"].to_string().trim_matches('"'))
        .unwrap()
        .to_string()
        .to_lowercase();

    let files = json["files"].as_array().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            "Failed to parse files array from JSON",
        )
    })?;

    let file_urls = files
        .iter()
        .filter_map(|file| file["url"].as_str().map(|url| url.to_string()))
        .collect::<Vec<_>>();

    let file_url = &file_urls[0];

    let response = reqwest::get(file_url)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, format!("Request failed: {:?}", err)))?;

    let config = read_config().unwrap();

    let file_name = format!(
        "{}/{}-{}.jar",
        config.mc_mod_dir,
        name.replace("-", "_"),
        config.minecraft_data.version
    );
    let mut file = File::create(&file_name).await?;
    let mut content = response.bytes().await.map_err(|err| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to read response: {:?}", err),
        )
    })?;
    file.write_all(&mut content).await.map_err(|err| {
        Error::new(
            ErrorKind::Other,
            format!("Failed to write to file: {:?}", err),
        )
    })?;

    Ok(())
}

async fn download_mod(
    json_str: &str,
    mc_version: &str,
    mod_loader: ModLoader,
) -> Result<(), Box<dyn std::error::Error>> {
    let json: Value = serde_json::from_str(json_str)?;

    let binding = json["slug"].as_str().unwrap_or("").trim_matches('"');
    let mod_versions = get_mod_versions(binding).await?;
    if let Some(mod_version) = mod_versions
        .iter()
        .find(|v| v.minecraft_version == mc_version && v.loader.contains(&mod_loader))
    {
        let response = reqwest::get(&mod_version.download_url)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, format!("Request failed: {:?}", err)))?;

        let config = read_config().unwrap();

        let file_name = format!(
            "{}/{}-{}.jar",
            config.mc_mod_dir,
            binding.replace("-", "_"),
            mod_version.minecraft_version
        );
        let mut file = File::create(&file_name).await?;
        let mut content = response.bytes().await.map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to read response: {:?}", err),
            )
        })?;
        file.write_all(&mut content).await.map_err(|err| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to write to file: {:?}", err),
            )
        })?;

        println!("Installing dependencies...");

        // Install dependencies
        for dependency in mod_version.dependencies.iter() {
            let id = &dependency.version_id;
            if let Err(err) = install_dep(id).await {
                eprintln!("Error: {:?}", err);
            }
        }
        println!(
            "Successfully installed {} for Minecraft version {}",
            binding, mc_version
        );
    } else {
        println!(
            "Failed to install {} for Minecraft version {}",
            binding, mc_version
        );
    }

    Ok(())
}

#[async_trait]
impl Command for InstallCommand {
    async fn run(&self) {
        print!("Enter mod to install: ");
        flush_output_stream();
        let input = get_user_input().to_lowercase();
        let config = read_config().unwrap();
        let mc_version = config.minecraft_data.version;
        let loader = config.minecraft_data.mod_loader;

        println!("Installing {} for Minecraft version {}.", input, mc_version);

        match get_project(&input).await {
            Ok(json) => {
                if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                    if let Err(err) = download_mod(&pretty_json, &mc_version, loader).await {
                        eprintln!("Error: {:?}", err);
                    }
                } else {
                    println!("Failed to format JSON");
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    fn description(&self) -> &str {
        "install mods"
    }
}
