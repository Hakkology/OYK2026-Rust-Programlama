use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] ParseIntError),
}

fn read_and_parse_number(file_path: String) -> Result<i32, AppError> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn main() {
    let file = String::from("number.txt");
    match read_and_parse_number(file) {
        Ok(number) => println!("File contains number: {}", number),
        Err(e) => {
            println!("{:?}", e);
            match e {
                AppError::Io(_) => std::process::exit(1),
                AppError::Parse(_) => std::process::exit(2),
            }
        }
    }
}