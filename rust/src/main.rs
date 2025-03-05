extern crate nalgebra as na;

mod solutions;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    year: u16,
    day: Option<u8>,
}

fn main() {
    let cli = Cli::parse();
    let selected_day = solutions::get_solutions(cli.year, cli.day);

    for sol in selected_day.into_iter() {
        match sol {
            Some(mut s) => s.run(),
            None => println!("Solution for day is not yet implemented"),
        }
    }
}
