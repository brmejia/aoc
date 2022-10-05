use std::str::FromStr;

use anyhow::Result;

pub fn parse_input_lines<T: FromStr>(input: &String) -> Result<Vec<T>> {
    Ok(input
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn split_line<T: FromStr>(line: &String, sep: char) -> Result<Vec<T>> {
    return Ok(line
        .split(sep)
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<T>>());
}
