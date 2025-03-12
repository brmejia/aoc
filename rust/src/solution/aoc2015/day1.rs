use crate::solution::{ PartResult, Solution};

#[derive(Debug)]
pub struct Day1 {}

impl Day1 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Day1 {
    fn part1(&mut self, input: String) -> PartResult {
        let ups = input.matches("(").count();
        let downs = input.matches(")").count();

        let floor = ups - downs;

        Ok(vec![floor.to_string()])
    }

    fn part2(&mut self, input: String) -> PartResult {
        let expected_floor = -1;

        let mut floor = 0;
        let mut idx = 0;
        for (_idx, char) in input.chars().enumerate() {
            match char {
                '(' => {
                    floor += 1;
                }
                ')' => {
                    floor -= 1;
                }
                _ => (),
            };
            if floor == expected_floor {
                idx = _idx;
                break;
            }
        }

        Ok(vec![idx.to_string()])
    }
}
