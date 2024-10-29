use std::fs;

pub fn parse_input(path: &str) -> String {
    fs::read_to_string(path).expect(&format!(
        "Should have been able to read the file in path {path}"
    ))
}

pub mod year_2023 {
    pub mod day_01;
}
