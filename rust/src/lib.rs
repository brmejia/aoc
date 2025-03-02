use anyhow::Result;
use std::str::FromStr;

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
