use std::{
    fs::{self, File},
    io::{Read, Write, copy},
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};
use zip::{write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::lib::modify::config_helper::read_config;

pub(crate) fn get_backup_dir() -> PathBuf {
    let settings = read_config().unwrap();
    let mc_folder: &Path;

    let path = Path::new(&settings.mc_mod_dir);
    if let Some(parent) = path.parent() {
        mc_folder = parent;
    } else {
        println!("Unable to get minecraft directory!");
        // Return a default backup directory path or any other appropriate path.
        return PathBuf::from(&settings.mc_mod_dir);
    }

    let backup_folder_str = format!("{}/{}", mc_folder.display(), "mod-backups");
    PathBuf::from(backup_folder_str)
}

pub(crate) fn zip_folder(
    input_folder: &Path,
    output_zip: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = File::create(output_zip)?;
    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(fs::metadata(input_folder)?.permissions().mode());

    zip_folder_recursive(&input_folder, &mut zip, &input_folder, options)?;

    Ok(())
}

pub(crate) fn unzip_file(
    zip_file_path: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the ZIP archive
    let zip_file = File::open(zip_file_path)?;
    let mut zip_archive = ZipArchive::new(zip_file)?;

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Extract each file from the ZIP archive
    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let file_name = file.sanitized_name();

        // Create the output file path
        let output_path = format!("{}/{}", output_dir, file_name.to_string_lossy());

        // Create the output file
        let mut output_file = File::create(&output_path)?;

        // Read the file data from the ZIP archive and write it to the output file
        copy(&mut file, &mut output_file)?;
    }

    Ok(())
}

fn zip_folder_recursive(
    input_folder: &Path,
    zip: &mut ZipWriter<File>,
    base_folder: &Path,
    options: FileOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(input_folder)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .strip_prefix(base_folder)?
            .to_string_lossy()
            .into_owned();

        if path.is_dir() {
            zip.add_directory(name, options)?;
            zip_folder_recursive(&path, zip, base_folder, options)?;
        } else {
            zip.start_file(name, options)?;
            let mut file = File::open(&path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
    }

    Ok(())
}
