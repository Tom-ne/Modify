use serde::Deserialize;
use std::io;

use super::request_handler::make_request;

#[derive(Deserialize)]
struct VersionInfo {
    version: String,
}

pub(crate) async fn get_mc_versions() -> Result<Vec<String>, io::Error> {
    let req = String::from("https://api.modrinth.com/v2/tag/game_version");
    let headers = String::new();

    let json = make_request(req.to_owned(), headers).await.map_err(|err| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to get Minecraft versions! {}", err),
        )
    })?;

    let versions: Result<Vec<String>, io::Error> = json
        .as_array()
        .map(|array| {
            array
                .iter()
                .map(|obj| {
                    let version_info: VersionInfo =
                        serde_json::from_value(obj.clone()).map_err(|err| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                format!("Failed to deserialize version info: {}", err),
                            )
                        })?;
                    Ok(version_info.version)
                })
                .collect()
        })
        .unwrap_or(Ok(Vec::new()));

    versions
}
