use serde_json::{Value, Error};

use crate::{
    input_helper::{flush_output_stream, get_user_input},
    api::make_request,
};

fn print_mod_info(json_str: &str) -> Result<(), Error> {
    let json: Value = serde_json::from_str(json_str)?;

    if let Some(hits) = json["hits"].as_array() {
        for hit in hits {
            let title = hit["title"].as_str().unwrap_or("");
            let description = hit["description"].as_str().unwrap_or("");
            let project_type = hit["project_type"].as_str().unwrap_or("");
            let client_side = hit["client_side"].as_str().unwrap_or("");
            let server_side = hit["server_side"].as_str().unwrap_or("");

            let versions = hit["versions"]
                .as_array()
                .map(|v| v.iter().map(|version| version.as_str().unwrap_or("")).collect::<Vec<&str>>())
                .unwrap_or_else(Vec::new);

            let separator = "==============================================";
            println!("{}", separator);

            // Calculate the spacing to center the title
            let separator_length = separator.chars().count();
            let title_length = title.chars().count();
            let left_spacing = (separator_length - title_length) / 2;
            let right_spacing = separator_length - title_length - left_spacing;

            println!("{}{}{}", " ".repeat(left_spacing), title, " ".repeat(right_spacing));
            println!("{}", separator);
            println!("• {}", description);
            println!("• Type: {}", project_type);
            println!("• Client side: {}", client_side);
            println!("• Server side: {}", server_side);
            println!("• Versions: {}", versions.join(", "));
        }
    }

    Ok(())
}

pub(crate) async fn run() {
    print!("Enter mod to search for: ");
    flush_output_stream();
    let input = get_user_input();
    let req = format!("https://api.modrinth.com/v2/search?query={}", input);
    let headers = String::new();

    match make_request(req, headers).await {
        Ok(json) => {
            if let Ok(pretty_json) = serde_json::to_string_pretty(&json) {
                if let Err(err) = print_mod_info(&pretty_json) {
                    eprintln!("Error: {:?}", err);
                }
            } else {
                println!("Failed to format JSON");
            }
        },
        Err(err) => {
            eprintln!("Error: {:?}", err)
        }
    }
}
