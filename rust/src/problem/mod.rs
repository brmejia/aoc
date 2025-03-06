use crate::{
    error::Result,
    solution::{DaySolution, try_get_day_solution},
};

pub struct Problem {
    year: u16,
    day: u8,
    pub solution: Box<dyn DaySolution>,
}

impl Problem {
    // add code here
    pub fn new(year: u16, day: u8) -> Result<Self> {
        let solution = try_get_day_solution(year, day)?;

        Ok(Self {
            year,
            day,
            solution,
        })
    }

    // pub fn solve() -> Result<Self> {
    //     let solution = try_get_day_solution(year, day)?;
    //
    //     Ok(Self {
    //         year,
    //         day,
    //         solution,
    //     })
    // }

    pub fn get_year(&self) -> u16 {
        self.year
    }
    pub fn get_day(&self) -> u8 {
        self.day
    }
}
