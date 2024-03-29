use crate::solutions::{Day, DaySolution, PartResult, Solution};

#[derive(Debug)]
pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Day for Day1 {
    fn get_year(&self) -> usize {
        return 2015;
    }
    fn get_day(&self) -> usize {
        return 1;
    }
}

impl Solution for Day1 {
    fn part1(&mut self) -> PartResult {
        let input = self.get_input(None);
        let ups = input.matches("(").count();
        let downs = input.matches(")").count();

        let floor = ups - downs;

        return Ok(vec![floor.to_string()]);
    }

    fn part2(&mut self) -> PartResult {
        let input = self.get_input(None);

        let expected_floor = -1;

        let mut floor = 0;
        let mut idx = 0;
        for (_idx, char) in input.chars().into_iter().enumerate() {
            match char {
                '(' => {
                    floor = floor + 1;
                }
                ')' => {
                    floor = floor - 1;
                }
                _ => (),
            };
            if floor == expected_floor {
                idx = _idx;
                break;
            }
        }

        return Ok(vec![idx.to_string()]);
    }
}

impl DaySolution for Day1 {}
