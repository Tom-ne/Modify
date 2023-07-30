use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

use crate::lib::{
    io::io_helper::{flush_output_stream, get_user_input},
    modify::{
        backup::backup_helper::{get_backup_dir, unzip_file},
        command::Command,
        config_helper::read_config,
    },
};

pub struct LoadBackupCommand;

struct BackupInfo {
    timestamp: NaiveDateTime,
    path: PathBuf,
}

fn list_zip_files(dir_path: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir_path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && path.extension().unwrap_or_default() == "zip" {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn parse_timestamp_from_filename(file_path: &Path) -> NaiveDateTime {
    let file_name = file_path.file_stem().unwrap().to_string_lossy();
    let parts: Vec<&str> = file_name.split('-').collect();

    let year: i32 = parts[4].parse().unwrap();
    let month: u32 = parts[3].parse().unwrap();
    let day: u32 = parts[2].parse().unwrap();
    let hour: u32 = parts[0].parse().unwrap();
    let minute: u32 = parts[1].parse().unwrap();

    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap_or_else(|| NaiveDate::from_ymd(1970, 1, 1)),
        NaiveTime::from_hms_opt(hour, minute, 0).unwrap_or_else(|| NaiveTime::from_hms(0, 0, 0)),
    )
}

fn remove_directory_contents(dir_path: &str) -> Result<(), std::io::Error> {
    if !fs::metadata(dir_path)?.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            remove_directory_contents(&path.to_string_lossy())?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

#[async_trait]
impl Command for LoadBackupCommand {
    async fn run(&self) {
        let backup_folder = get_backup_dir();

        if !backup_folder.exists() {
            println!("You do not have any backups to restore from!");
            return;
        }

        let zip_files = list_zip_files(&backup_folder);
        let mut backup_files_with_timestamp: Vec<_> = zip_files
            .iter()
            .map(|file| (file.clone(), parse_timestamp_from_filename(file)))
            .collect();

        backup_files_with_timestamp.sort_by(|a, b| b.1.cmp(&a.1));

        let backups: Vec<BackupInfo> = backup_files_with_timestamp
            .into_iter()
            .map(|(path, timestamp)| BackupInfo { timestamp, path })
            .collect();

        for (backup_number, backup_info) in backups.iter().enumerate() {
            let local_time = Local
                .from_local_datetime(&backup_info.timestamp)
                .single()
                .unwrap();
            println!(
                "Backup {}: {} at {}",
                backup_number + 1,
                local_time.format("%Y-%m-%d"),
                local_time.format("%H:%M:%S")
            );
        }

        loop {
            print!("Select backup number to restore from: ");
            flush_output_stream();
            let input = get_user_input().to_lowercase();

            if let Ok(backup_index) = input.parse::<usize>() {
                if backup_index > 0 && backup_index <= backups.len() {
                    // Now, you have the selected backup index in 'backup_index', which is a valid backup number.
                    // You can use this index to retrieve the corresponding backup information from the 'backups' vector.
                    let selected_backup = backups.get(backup_index - 1).unwrap();
                    println!("Selected backup: {:?}", selected_backup.path);

                    print!("This action will remove all mods from your current mods folder, would you like to continue (Y/n)? ");
                    flush_output_stream();
                    let should_continue = get_user_input().contains('Y');

                    if should_continue {
                        let settings = read_config().unwrap();
                        println!("removing contents of old mods directory...");
                        let _ = remove_directory_contents(&settings.mc_mod_dir);

                        println!("loading backup...");
                        let _ = unzip_file(
                            selected_backup.path.to_str().unwrap(),
                            &settings.mc_mod_dir,
                        );
                        println!("loaded backup successfully");
                    } else {
                        println!("Ok, I won't do anything.");
                    }

                    break;
                } else {
                    println!("Invalid backup number. Please select a valid backup number.");
                }
            } else {
                println!("Invalid input. Please enter a number for the backup selection.");
            }
        }
    }

    fn description(&self) -> &str {
        "load mods from backup"
    }
}
