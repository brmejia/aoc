// use itertools::Itertools;
// use std::collections::BTreeMap;

use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;

use super::DaySolution;

const UPPER_DAYS_LIMIT: u8 = 9;

pub fn get_day_solution(day: u8) -> Result<Option<Box<dyn DaySolution>>> {
    let day_solution: Option<Box<dyn DaySolution>> = match day {
        1 => Some(Box::new(Day1::new())),
        2 => Some(Box::new(Day2::new())),
        3 => Some(Box::new(Day3::new())),
        4 => Some(Box::new(Day4::new())),
        5 => Some(Box::new(Day5::new())),
        6 => Some(Box::new(Day6::new())),
        7 => Some(Box::new(Day7::new())),
        8 => Some(Box::new(Day8::new())),
        UPPER_DAYS_LIMIT..=24 => None,
        0 | 25.. => panic!("Day {:?} is non in Advent of Code", day),
    };
    Ok(day_solution)
}

pub fn get_solutions(day: Option<u8>) -> Result<Vec<Option<Box<dyn DaySolution>>>> {
    match day {
        Some(d) => {
            let day_solution = get_day_solution(d)?;
            Ok(vec![day_solution])
        }
        _ => Ok((1..UPPER_DAYS_LIMIT)
            .flat_map(|d| get_solutions(d.into()))
            .flatten()
            .collect()),
    }
}
