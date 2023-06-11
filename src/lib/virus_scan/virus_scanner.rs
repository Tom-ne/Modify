use super::{scanner_windows::scan_windows, scanner_linux::scan_linux};

pub(crate) fn scan() {
    let os = std::env::consts::OS;

    match os {
        "windows" => scan_windows(),
        "linux" => scan_linux(),
        "macos" => println!("Could not perform virus scan!"),
        _ => println!("Unknown operating system: {}, could not perform virus scan!", os)
    }
}