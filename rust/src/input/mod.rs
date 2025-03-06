pub mod error;

use error::Result;
use std::{fs, path::Path, str::FromStr};

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

pub fn parse_file_lines<T: FromStr>(file_path: impl AsRef<Path>) -> Result<Vec<T>> {
    let file_content = fs::read_to_string(&file_path)
        // .context(format!(
        //     "Unable to read input file {:?}",
        //     file_path.as_ref()
        // ))
    ?;

    parse_input_lines(&file_content)
}
