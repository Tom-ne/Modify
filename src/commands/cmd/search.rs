use async_trait::async_trait;
use serde_json::{Error, Value};

use crate::lib::{
    io::io_helper::{flush_output_stream, get_user_input, print_middle},
    modify::command::Command,
    modrinth::search_req::search_mod,
};

pub struct SearchCommand;

fn print_mod_info(json_str: &str) -> Result<(), Error> {
    let json: Value = serde_json::from_str(json_str)?;

    if let Some(hits) = json["hits"].as_array() {
        for hit in hits {
            let slug = hit["slug"].as_str().unwrap_or("");
            let title = hit["title"].as_str().unwrap_or("");
            let description = hit["description"].as_str().unwrap_or("");
            let project_type = hit["project_type"].as_str().unwrap_or("");
            let client_side = hit["client_side"].as_str().unwrap_or("");
            let server_side = hit["server_side"].as_str().unwrap_or("");

            let versions = hit["versions"]
                .as_array()
                .map(|v| {
                    v.iter()
                        .map(|version| version.as_str().unwrap_or(""))
                        .collect::<Vec<&str>>()
                })
                .unwrap_or_else(Vec::new);

            let separator = "==============================================";
            print_middle(separator, title);
            println!("• {}", description);
            println!("• Slug: {}", slug);
            println!("• Type: {}", project_type);
            println!("• Client side: {}", client_side);
            println!("• Server side: {}", server_side);
            println!("• Versions: {}", versions.join(", "));
        }
    }

    Ok(())
}

#[async_trait]
impl Command for SearchCommand {
    async fn run(&self) {
        print!("Enter mod to search for: ");
        flush_output_stream();
        let input = get_user_input();

        match search_mod(&input).await {
            Ok(pretty_json) => {
                if let Err(err) = print_mod_info(&pretty_json) {
                    eprintln!("Error: {:?}", err);
                }
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }

    fn description(&self) -> &str {
        "search for mods"
    }
}
