use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
    println!("Searching \"{query}\" in file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Failed to read file.");
    println!("found text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];
    (query, file_path)
}
