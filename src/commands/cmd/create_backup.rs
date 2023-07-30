use std::{fs, path::Path};

use async_trait::async_trait;
use chrono::{DateTime, Datelike, Local, Timelike};

use crate::lib::modify::{command::Command, config_helper::read_config, backup::backup_helper::{get_backup_dir, zip_folder}};

pub struct CreateBackupCommand;

#[async_trait]
impl Command for CreateBackupCommand {
    async fn run(&self) {
        let settings = read_config().unwrap();

        let local_time: DateTime<Local> = Local::now();
        let formatted_time = format!(
            "{:02}-{:02}-{:02}-{:02}-{}",
            local_time.hour(),
            local_time.minute(),
            local_time.day(),
            local_time.month(),
            local_time.year()
        );

        let backup_folder = get_backup_dir();

        if let Err(e) = fs::create_dir_all(backup_folder.clone()) {
            println!("Error creating backup folder: {:?}", e);
            return;
        }

        let zip_file_path = format!("{}.zip", formatted_time);
        let output_zip = backup_folder.join(zip_file_path);

        println!("Creating backup...");
        if let Err(e) = zip_folder(Path::new(&settings.mc_mod_dir), &output_zip) {
            println!("Failed to create backup: {:?}", e);
        } else {
            println!("Backup created successfully");
        }
    }

    fn description(&self) -> &str {
        "create a backup of the mods folder"
    }
}
