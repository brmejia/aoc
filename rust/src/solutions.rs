use anyhow::Result;
use std::fs;

pub mod day1;
pub mod day2;

const INPUT_BASE_PATH: &str = "../inputs/";

type PartResult = Result<Vec<String>>;

pub trait DaySolution {
    fn get_year(&self) -> usize;

    fn get_day(&self) -> usize;

    fn get_name(&self) -> String {
        return format!("{} Day {}", self.get_year(), self.get_day());
    }

    fn get_input_path(&self) -> String {
        let mut input_path = INPUT_BASE_PATH.to_string();
        input_path.push_str(&format!("day{}.txt", self.get_day()));
        return input_path;
    }

    fn get_input(&self, input_path: Option<&String>) -> String {
        let input_path = input_path.unwrap_or(&self.get_input_path()).to_owned();

        let input = fs::read_to_string(input_path).expect("Unable to read input file");
        return input;
    }

    fn part1(&self) -> PartResult;

    fn part2(&self) -> PartResult;
}

pub fn print_solution(title: &str, lines: Vec<String>) {
    println!("{}", title);
    for line in lines.iter() {
        println!("\t{}", &line);
    }
}

pub fn run_solution(solution: &Box<dyn DaySolution>) {
    println!("---- {} ----", solution.get_name());
    let part1 = solution.part1().unwrap();
    print_solution("Part 1:", part1);
    let part2 = solution.part2().unwrap();
    print_solution("Part 2:", part2);
}
