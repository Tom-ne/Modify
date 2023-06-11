use std::{env, fs};

pub(crate) fn scan_windows() {
    let app_data = format!("{}/AppData", env::var("HOME").unwrap());
    let edge_path = format!("{}/Local/Microsoft Edge", app_data);

    let bad_paths = vec![
        format!("{}/.ref", edge_path),
        format!("{}/client.jar", edge_path),
        format!("{}/lib.dll", edge_path),
        format!("{}/libWebGL64.jar", edge_path),
        format!("{}/run.bat", edge_path),
        format!(
            "{}/Roaming/Microsoft/Windows/Start Menu/Programs/Startup/run.bat",
            app_data
        ),
        "HKCU:\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\\t".to_string(),
    ];

    let mut res = false;
    for path in bad_paths {
        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.is_file() {
                println!("bad file found! removing {}...", &path);
                if let Err(err) = fs::remove_file(&path) {
                    eprintln!("Failed to remove bad file {}: {}", &path, err);
                }
                res = true
            }
        }
    }
}
