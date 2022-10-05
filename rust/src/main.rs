mod solutions;
use solutions::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, DaySolution};

fn main() {
    let mut days: Vec<Box<dyn DaySolution>> = vec![
        Box::new(Day1 {}),
        Box::new(Day2 {}),
        Box::new(Day3 {}),
        Box::new(Day4::new()),
    ];

    for day in days.iter_mut() {
        solutions::run_solution(day)
    }
}
