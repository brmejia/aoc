pub mod error;

use error::Result;
use std::{fs, path::Path, str::FromStr};

const INPUT_BASE_PATH: &str = "../inputs/";

pub fn parse_input_lines<T: FromStr>(input: &str) -> Result<Vec<T>> {
    Ok(input
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn split_line<T: FromStr>(line: &str, sep: char) -> Result<Vec<T>> {
    Ok(line
        .split(sep)
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<T>>())
}

#[allow(dead_code)]
pub fn parse_file_lines<T: FromStr>(file_path: impl AsRef<Path>) -> Result<Vec<T>> {
    let file_content = fs::read_to_string(&file_path)?;

    parse_input_lines(&file_content)
}

pub fn get_default_input_path(year: u16, day: u8) -> String {
    let mut input_path = INPUT_BASE_PATH.to_string();
    input_path.push_str(&format!("{}/day{}.txt", year, day));
    input_path
}
