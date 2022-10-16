use itertools::Itertools;
use std::collections::BTreeMap;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;

use super::DaySolution;

pub fn get_solutions(day: Option<u16>) -> Vec<Box<dyn DaySolution>> {
    let mut days: BTreeMap<u16, Box<dyn DaySolution>> = BTreeMap::new();
    days.insert(1, Box::new(Day1::new()));
    days.insert(2, Box::new(Day2::new()));
    days.insert(3, Box::new(Day3::new()));
    days.insert(4, Box::new(Day4::new()));
    days.insert(5, Box::new(Day5::new()));
    days.insert(6, Box::new(Day6::new()));

    match day {
        Some(day) => {
            let selected_day = days
                .remove(&day)
                .expect(format!("Day {day} of 2015 is not implemented yet").as_str());
            vec![selected_day]
        }
        _ => days.into_values().collect_vec(),
    }
}
