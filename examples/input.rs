use regex::Regex;
use zenity::menu::input::{valid_path, valid_regex};

fn main() {
    println!(
        "\n\nReturn:  {}",
        valid_regex(Regex::new(r"^\d{3}$").unwrap())
    );
    println!("\n\nPath:  {:?}", valid_path());
}