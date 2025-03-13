extern crate nalgebra as na;

mod error;
mod input;
mod problem;
mod solution;

use std::time::Instant;

use clap::Parser;
use problem::Problem;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    year: u16,
    day: Option<u8>,
}

fn print_solution(title: &str, lines: &[String]) {
    let title = title.trim();
    let indent_size = title.len() + 1;

    println!("{} {}", title, lines.first().unwrap());
    for line in lines.iter().skip(1) {
        println!("{}{}", " ".repeat(indent_size), &line);
    }
}

fn solve_problem(problem: Problem, input: String) {
    let _: Vec<_> = (1..=2)
        .map(|part_idx| {
            let current = Instant::now();
            let result = match part_idx {
                1 => problem.get_solution().part1(&input),
                2 => problem.get_solution().part2(&input),
                _ => todo!(),
            };
            let duration = current.elapsed();

            match result {
                Ok(r) => {
                    print_solution(format!("Part {part_idx}:").as_str(), &r);
                    println!("Elapsed time: {:?}\n", duration);
                    Ok(r)
                }
                Err(e) => Err(e),
            }
        })
        .collect();
}

fn main() {
    let cli = Cli::parse();

    let problems = match cli.day {
        Some(day) => vec![Problem::new(cli.year, day)],
        None => (1..=24).map(|d| Problem::new(cli.year, d)).collect(),
    };

    for problem in problems {
        match problem {
            Ok(p) => {
                println!("---- {} Day {} ----", p.get_year(), p.get_day());
                let input = p.get_default_input().expect("Error getting default input");
                // p.with_input(input).solve()
                solve_problem(p, input);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
