pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
pub fn read_input(file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(format!("src/inputs/{}", file))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}