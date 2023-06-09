use std::io;

use serde_json::Value;

use super::request_handler::make_request;

pub(crate) async fn get_project(query: &str) -> Result<Value, io::Error> {
    let url = format!("https://api.modrinth.com/v2/project/{}", query);
    let headers = String::new();

    make_request(url, headers)
        .await
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
}