use std::io::{self, Write};

pub(crate) fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

pub(crate) fn flush_output_stream() {
    io::stdout().flush().unwrap();
}
