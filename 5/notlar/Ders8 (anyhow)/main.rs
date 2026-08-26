use anyhow::{Context, Result};
use std::{fs::File, io::Read};


fn read_and_parse_number(file_path: String) -> Result<i32> {
    let mut file = File::open(file_path).context("Failed to read file contents")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number = contents
        .trim()
        .parse::<i32>()
        .with_context(|| format!("Failed to parse integer from contents: {}", contents.trim()))?;
    Ok(number)
}
fn main() {
    let file = String::from("number.txt");
    match read_and_parse_number(file) {
        Ok(number) => println!("File contains number: {}", number),
        Err(e) => println!("Cannot process: {:?}", e),
    }
}