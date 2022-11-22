pub mod aoc2015;

use anyhow::Result;
use core::fmt::Debug;
use std::{fs, time::Instant};

pub fn get_solution(year: u16, day: Option<u16>) -> Vec<Box<dyn DaySolution>> {
    let day_solution = match year {
        2015 => aoc2015::get_solutions(day),
        _ => panic!("Advent of Code {} is not already implemented", year),
    };

    return day_solution;
}

const INPUT_BASE_PATH: &str = "../inputs/";

type PartResult = Result<Vec<String>>;

pub trait Day {
    fn get_year(&self) -> usize;
    fn get_day(&self) -> usize;
    fn get_name(&self) -> String {
        return format!("{} Day {}", self.get_year(), self.get_day());
    }
}

pub fn get_input_path(year: u16, day: u8) -> String {
    let mut input_path = INPUT_BASE_PATH.to_string();
    input_path.push_str(&format!("{}/day{}.txt", year, day));
    return input_path;
}
pub fn get_problem_input(year: u16, day: u8) -> Option<String> {
    let input_path = get_input_path(year, day);
    let input = fs::read_to_string(input_path).expect("Unable to read input file");
    return Some(input);
}

pub trait Solution: Day {
    fn get_input(&self, input_path: Option<&String>) -> String {
        return get_problem_input(self.get_year() as u16, self.get_day() as u8).unwrap();
    }

    fn part1(&mut self) -> PartResult;

    fn part2(&mut self) -> PartResult;
}

pub trait DaySolution: Day + Solution + Debug {
    fn print_solution(&mut self, title: &str, lines: Vec<String>) {
        let title = title.trim();
        let indent_size = title.len() + 1;

        println!("{} {}", title, lines.first().unwrap());
        for line in lines.iter().skip(1) {
            println!("{}{}", " ".repeat(indent_size), &line);
        }
    }
    fn run(&mut self) {
        println!("---- {} ----", self.get_name());

        let current = Instant::now();
        let part1 = self.part1().unwrap();
        let duration1 = current.elapsed();
        self.print_solution("Part 1:", part1);
        println!("Elapsed time: {:?}\n", duration1);

        let current = Instant::now();
        let part2 = self.part2().unwrap();
        let duration2 = current.elapsed();
        self.print_solution("Part 2:", part2);
        println!("Elapsed time: {:?}\n", duration2);
    }
}
