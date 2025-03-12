use std::{cell::RefCell , fs};

use crate::{
    error::{AoCError, Result},
    input::get_default_input_path,
    solution::{Solution, try_get_day_solution},
};

pub struct Problem {
    year: u16,
    day: u8,
    solution: RefCell<Box<dyn Solution>>,
}

impl Problem {
    // add code here
    pub fn new(year: u16, day: u8) -> Result<Self> {
        let solution = try_get_day_solution(year, day)?;

        Ok(Self {
            year,
            day,
            solution: RefCell::new(solution),
        })
    }

    pub fn get_default_input(&self) -> Result<String> {
        let input_path = get_default_input_path(self.year, self.day);
        fs::read_to_string(input_path).map_err(AoCError::IO)
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
    pub fn get_solution(&self) -> &RefCell<Box<dyn Solution>> {
        &self.solution
    }
}
