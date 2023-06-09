use std::io::{self, Write, stdout};

use termion::clear;

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

pub(crate) fn clear() {
    let mut stdout = stdout();
    write!(stdout, "{}", clear::All).unwrap();
    stdout.flush().unwrap();
}

pub(crate) fn print_middle(separator: &str, title: &str) {
    println!("{}", separator);

    // Calculate the spacing to center the title
    let separator_length = separator.chars().count();
    let title_length = title.chars().count();
    let left_spacing = (separator_length - title_length) / 2;
    let right_spacing = separator_length - title_length - left_spacing;

    println!("{}{}{}", " ".repeat(left_spacing), title, " ".repeat(right_spacing));
    println!("{}", separator);
}
