pub mod aoc2015;
pub mod error;

use core::fmt::Debug;
use error::{Result, SolutionError};
use std::{collections::HashMap, fs, time::Instant};

use crate::input::get_default_input_path;

pub fn try_get_day_solution(year: u16, day: u8) -> Result<Box<dyn DaySolution>> {
    match year {
        2015 => aoc2015::try_get_day_solution(day),
        _ => Err(SolutionError::NotImplementedYear { year }),
    }
}

pub fn get_solutions(year: u16, day: Option<u8>) -> HashMap<u8, Result<Box<dyn DaySolution>>> {
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
    fn part1(&mut self, input: String) -> PartResult;

    fn part2(&mut self, input: String) -> PartResult;
}

pub trait DaySolution: Solution + Debug {
    // fn print_solution(&mut self, title: &str, lines: &Vec<String>) {
    fn print_solution(&mut self, title: &str, lines: &[String]) {
        let title = title.trim();
        let indent_size = title.len() + 1;

        println!("{} {}", title, lines.first().unwrap());
        for line in lines.iter().skip(1) {
            println!("{}{}", " ".repeat(indent_size), &line);
        }
    }
    fn solve(&mut self, input: String) {
        let _: Vec<_> = (1..=2)
            .map(|part_idx| {
                let current = Instant::now();
                let result = match part_idx {
                    1 => self.part1(input.clone()),
                    2 => self.part2(input.clone()),
                    _ => todo!(),
                };
                let duration = current.elapsed();

                match result {
                    Ok(r) => {
                        self.print_solution(format!("Part {part_idx}:").as_str(), &r);
                        println!("Elapsed time: {:?}\n", duration);
                        Ok(r)
                    }
                    Err(e) => Err(e),
                }
            })
            .collect();
    }
}
