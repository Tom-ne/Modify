use super::request_handler::make_request;

pub(crate) async fn search_mod(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let req = format!("https://api.modrinth.com/v2/search?query={}&limit=5", query);
    let headers = String::new();

    match make_request(req, headers).await {
        Ok(json) => {
            if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                Ok(pretty_json)
            } else {
                Err("Failed to format JSON".into())
            }
        }
        Err(err) => Err(err.into()),
    }
}
