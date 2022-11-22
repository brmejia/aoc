extern crate nalgebra as na;

mod solutions;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    year: u16,
    day: Option<u16>,
}

fn main() {
    let cli = Cli::parse();
    let selected_day = solutions::get_solution(cli.year, cli.day);

    for mut sol in selected_day.into_iter() {
        sol.run()
    }
}
