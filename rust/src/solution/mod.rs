pub mod aoc2015;
pub mod error;

use error::{Result, SolutionError};
use std::collections::HashMap;

pub fn try_get_day_solution(year: u16, day: u8) -> Result<Box<dyn Solution>> {
    match year {
        2015 => aoc2015::try_get_day_solution(day),
        _ => Err(SolutionError::NotImplementedYear { year }),
    }
}

pub fn get_solutions(year: u16, day: Option<u8>) -> HashMap<u8, Result<Box<dyn Solution>>> {
    match year {
        2015 => match day {
            Some(d) => {
                let mut solutions = HashMap::new();
                let sol = aoc2015::try_get_day_solution(d);
                solutions.insert(d, sol);
                solutions
            }
            None => aoc2015::get_solutions(),
        },
        _ => panic!("Advent of Code {} is not already implemented", year),
    }
}

type PartResult = Result<Vec<String>>;

pub trait Solution {
    fn part1(&self, input: &str) -> PartResult;

    fn part2(&self, input: &str) -> PartResult;
}
