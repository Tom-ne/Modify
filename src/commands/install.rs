use serde_json::Value;
use std::io::{self, Error, ErrorKind};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{
    input_helper::{flush_output_stream, get_user_input},
    api::make_request,
};

struct ModVersion {
    id: String,
    minecraft_version: String,
    download_url: String,
}

// Fetches the list of mod versions from the JSON response
async fn list_versions(json: &Value) -> Result<Vec<ModVersion>, io::Error> {
    let mut mod_versions = Vec::new();

    if let Some(versions) = json.as_array() {
        for version in versions {
            // Extract the ID, game versions, and download URL for each version
            let id = version["id"].as_str().unwrap_or("").to_string();
            let game_versions = version["game_versions"]
                .as_array()
                .map(|versions| {
                    versions
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let download_url = version["files"]
                .as_array()
                .and_then(|files| {
                    files
                        .iter()
                        .filter_map(|file| file["url"].as_str().map(|url| url.to_string()))
                        .next()
                })
                .unwrap_or_default();

            // Create a ModVersion instance for each game version and add it to the list
            for minecraft_version in game_versions {
                let mod_version = ModVersion {
                    id: id.clone(),
                    minecraft_version: minecraft_version.clone(),
                    download_url: download_url.clone(),
                };
                mod_versions.push(mod_version);
            }
        }
    }

    Ok(mod_versions)
}

// Fetches the mod versions for a given mod name
async fn get_mod_versions(mod_name: &str) -> Result<Vec<ModVersion>, io::Error> {
    let req = format!("https://api.modrinth.com/v2/project/{}/version", mod_name);
    let headers = String::new();

    match make_request(req, headers).await {
        Ok(json) => {
            // Convert the JSON response to a pretty formatted string
            if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                // Parse the pretty JSON string into a Value
                if let Ok(value) = serde_json::from_str(&pretty_json) {
                    // Fetch the list of mod versions from the parsed JSON value
                    let mod_versions = list_versions(&value).await?;
                    return Ok(mod_versions);
                } else {
                    return Err(Error::new(ErrorKind::Other, "Failed to parse JSON"));
                }
            } else {
                return Err(Error::new(ErrorKind::Other, "Failed to format JSON"));
            }
        },
        Err(err) => {
            return Err(Error::new(ErrorKind::Other, format!("Error: {:?}", err)));
        }
    }
}

// Downloads the mod file for the specified Minecraft version
async fn download_mod(json_str: &str, mc_version: &str) -> Result<(), io::Error> {
    let json: Value = serde_json::from_str(json_str)?;

    let mut has_installed = false;
    let binding = json["slug"].to_string();
    let mod_name = binding.trim_matches('"');
    let mod_versions = get_mod_versions(mod_name).await?;
    for mod_version in mod_versions {
        if mod_version.minecraft_version == mc_version {
            // Download the file
            let response = reqwest::get(&mod_version.download_url)
                .await
                .map_err(|err| Error::new(ErrorKind::Other, format!("Request failed: {:?}", err)))?;
            let file_name = format!("{}_{}.jar", mod_name, mod_version.minecraft_version);
            let mut file = File::create(file_name).await?;
            let mut content = response.bytes().await.map_err(|err| Error::new(ErrorKind::Other, format!("Failed to read response: {:?}", err)))?;
            file.write_all(&mut content).await.map_err(|err| Error::new(ErrorKind::Other, format!("Failed to write to file: {:?}", err)))?;
            has_installed = true;
        }
    }

    if !has_installed {
        println!("Failed to install {} for version {}", mod_name, mc_version);
    }

    Ok(())
}

// Entry point of the program
pub(crate) async fn run() {
    print!("Enter mod to install: ");
    flush_output_stream();
    let input = get_user_input().to_lowercase();
    print!("Enter mc version: ");
    flush_output_stream();
    let mc_version = get_user_input().to_lowercase();
    let req = format!("https://api.modrinth.com/v2/project/{}", input);
    let headers = String::new();

    match make_request(req, headers).await {
        Ok(json) => {
            if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                // Download the mod file
                if let Err(err) = download_mod(&pretty_json, &mc_version).await {
                    eprintln!("Error: {:?}", err);
                }
            } else {
                println!("Failed to format JSON");
            }
        },
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
