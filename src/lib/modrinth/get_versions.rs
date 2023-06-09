use std::io;

use serde_json::Value;

use super::request_handler::make_request;

pub(crate) struct ModVersion {
    pub minecraft_version: String,
    pub download_url: String,
}

async fn list_versions(json: &Value) -> Result<Vec<ModVersion>, io::Error> {
    let mut mod_versions = Vec::new();

    if let Some(versions) = json.as_array() {
        for version in versions {
            if let (Some(game_versions), Some(download_url)) = (
                version["game_versions"].as_array(),
                version["files"]
                    .as_array()
                    .and_then(|files| files[0]["url"].as_str()),
            ) {
                let minecraft_versions = game_versions
                    .iter()
                    .map(|v| v.as_str().unwrap_or("").to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                let mod_version = ModVersion {
                    minecraft_version: minecraft_versions,
                    download_url: download_url.to_string(),
                };
                mod_versions.push(mod_version);
            }
        }
    }

    Ok(mod_versions)
}

pub(crate) async fn get_mod_versions(mod_name: &str) -> Result<Vec<ModVersion>, io::Error> {
    let req = format!("https://api.modrinth.com/v2/project/{}/version", mod_name);
    let headers = String::new();

    let json = make_request(req, headers)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Error: {:?}", err)))?;

    let pretty_json = serde_json::to_string_pretty(&json)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to format JSON"))?;

    let value = serde_json::from_str(&pretty_json)
        .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to parse JSON"))?;

    let mod_versions = list_versions(&value).await?;
    Ok(mod_versions)
}
