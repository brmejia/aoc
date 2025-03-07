extern crate nalgebra as na;

mod error;
mod input;
mod problem;
mod solution;

use clap::Parser;
use input::get_default_input_path;
use problem::Problem;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    year: u16,
    day: Option<u8>,
}

fn main() {
    let cli = Cli::parse();

    let problems = match cli.day {
        Some(day) => vec![Problem::new(cli.year, day)],
        None => (1..=24).map(|d| Problem::new(cli.year, d)).collect(),
    };

    for problem in problems {
        match problem {
            Ok(mut p) => {
                println!("---- {} Day {} ----", p.get_year(), p.get_day());
                let input = p.get_default_input().expect("Error getting default input");
                p.solution.solve(input);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
