use std::fs;

pub fn parse_input(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

pub mod commands;

pub mod common {
    pub mod parsing;
}

pub mod year_2023 {
    pub mod day_01;
    pub mod day_02;
    pub mod day_03;
    pub mod day_04;
    pub mod day_05;
}
