mod solutions;
use solutions::{day1::Day1, day2::Day2, day3::Day3, DaySolution};

fn main() {
    let days: Vec<Box<dyn DaySolution>> =
        vec![Box::new(Day1 {}), Box::new(Day2 {}), Box::new(Day3 {})];

    for day in days.iter() {
        solutions::run_solution(day)
    }
}
