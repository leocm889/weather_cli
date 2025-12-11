use std::io;

use colored::*;
use thiserror::Error;

pub fn read_input<T: std::str::FromStr>() -> T {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();
        match trimmed.parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                println!("{}", "⚠️ Invalid input, try again.".yellow().bold());
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}
