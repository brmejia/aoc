// use itertools::Itertools;
// use std::collections::BTreeMap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use std::collections::HashMap;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;

use super::{
    DaySolution,
    error::{Result, SolutionError},
};

const UPPER_DAYS_LIMIT: u8 = 9;
/// Get the solution for the day
///
/// # Errors
///
/// This function will return an error if the day is not implemented or if the day is out of the range of 1-25.
///
pub fn try_get_day_solution(day: u8) -> Result<Box<dyn DaySolution>> {
    let day_solution: Result<Box<dyn DaySolution>> = match day {
        1 => Ok(Box::new(Day1::new())),
        2 => Ok(Box::new(Day2::new())),
        3 => Ok(Box::new(Day3::new())),
        4 => Ok(Box::new(Day4::new())),
        5 => Ok(Box::new(Day5::new())),
        6 => Ok(Box::new(Day6::new())),
        7 => Ok(Box::new(Day7::new())),
        8 => Ok(Box::new(Day8::new())),
        UPPER_DAYS_LIMIT..=24 => Err(SolutionError::NotImplementedDay { year: 2015, day }),
        0 | 25.. => Err(SolutionError::InvalidDay { day }),
    };
    day_solution
}

pub fn get_solutions() -> HashMap<u8, Result<Box<dyn DaySolution>>> {
    let mut solutions = HashMap::new();
    for day in 1..UPPER_DAYS_LIMIT {
        solutions.insert(day, try_get_day_solution(day));
    }
    solutions
}
