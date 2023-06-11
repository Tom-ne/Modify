use std::{env, fs};

pub(crate) fn scan_linux() {
    let service_file = "systemd-utility";
    let data_dir = format!("{}/.config/.data", env::var("HOME").unwrap());
    let bad_paths = vec![
        format!("{}/.ref", data_dir),
        format!("{}/client.jar", data_dir),
        format!("{}/lib.jar", data_dir),
        format!(
            "{}/.config/systemd/user/{}",
            env::var("HOME").unwrap(),
            service_file
        ),
        format!("/etc/systemd/system/{}", service_file),
    ];

    let mut res = true;
    for path in bad_paths {
        if fs::metadata(&path).is_ok() {
            println!("bad file found! removing {}...", &path);
            if let Err(err) = fs::remove_file(&path) {
                eprintln!("Failed to remove file {}:{}", &path, err);
                res = false;
            }
        }
    }
}
