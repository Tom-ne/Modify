use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Result as JsonResult;

pub(crate) async fn make_request(req: String, headers: String) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client
        .get(&req)
        .headers(parse_headers(Some(&headers)))
        .send()
        .await?;

    let body = response.text().await?;

    // Deserialize the JSON response
    let json_response: JsonResult<serde_json::Value> = serde_json::from_str(&body);
    let json = json_response?;

    Ok(json)
}

fn parse_headers(headers: Option<&str>) -> HeaderMap {
    let mut header_map = HeaderMap::new();

    // Set default User-Agent header
    let user_agent: HeaderName = "User-Agent".parse().unwrap();
    // let user_agent_value: HeaderValue = "mod_manager".parse().unwrap();
    let user_agent_value: HeaderValue = env!("CARGO_PKG_NAME").parse().unwrap();
    header_map.insert(user_agent.clone(), user_agent_value);

    if let Some(headers) = headers {
        for line in headers.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let header_name = parts[0].trim();
                let header_value = parts[1].trim();
                let header_name: HeaderName = header_name.parse().unwrap();
                let header_value: HeaderValue = header_value.parse().unwrap();
                header_map.insert(header_name, header_value);
            }
        }
    }

    header_map
}
