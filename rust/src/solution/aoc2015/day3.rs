use std::collections::HashSet;

use crate::{
    input,
    solution::{DaySolution, PartResult, Solution},
};

#[derive(Debug)]
pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct HousePosition {
    x: isize,
    y: isize,
}

fn houses_visited_by_santa(input: &str) -> usize {
    let mut santa_pos = HousePosition { x: 0, y: 0 };
    let mut visited_houses = HashSet::new();

    visited_houses.insert(santa_pos);

    for char in input.chars() {
        match char {
            '>' => santa_pos.x += 1,
            '<' => santa_pos.x -= 1,
            '^' => santa_pos.y += 1,
            'v' => santa_pos.y -= 1,
            _ => panic!("wrong instructions"),
        }
        visited_houses.insert(santa_pos);
    }
    visited_houses.len()
}

fn houses_visited_by_robo_santa(input: &str) -> usize {
    let mut santa_pos = HousePosition { x: 0, y: 0 };
    let mut robo_santa_pos = HousePosition { x: 0, y: 0 };

    let mut visited_houses = HashSet::new();
    visited_houses.insert(santa_pos);

    let mut actor_ptr: &mut HousePosition;
    for (idx, char) in input.chars().enumerate() {
        if idx % 2 == 0 {
            actor_ptr = &mut santa_pos;
        } else {
            actor_ptr = &mut robo_santa_pos;
        }
        match char {
            '>' => actor_ptr.x += 1,
            '<' => actor_ptr.x -= 1,
            '^' => actor_ptr.y += 1,
            'v' => actor_ptr.y -= 1,
            _ => panic!("wrong instructions"),
        }
        visited_houses.insert(*actor_ptr);
    }
    visited_houses.len()
}

impl Solution for Day3 {
    fn part1(&mut self, input: String) -> PartResult {
        let lines = input::parse_input_lines::<String>(&input).unwrap();

        Ok(lines
            .into_iter()
            .map(|line| houses_visited_by_santa(&line).to_string())
            .collect())
    }

    fn part2(&mut self, input: String) -> PartResult {
        let lines = input::parse_input_lines::<String>(&input).unwrap();

        Ok(lines
            .into_iter()
            .map(|line| houses_visited_by_robo_santa(&line).to_string())
            .collect())
    }
}

impl DaySolution for Day3 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_houses_visited_by_santa() {
        let validations = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (input, expected_result) in validations.into_iter() {
            let result = houses_visited_by_santa(input);

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_count_houses_visited_by_robo_santa() {
        let validations = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (input, expected_result) in validations.into_iter() {
            let result = houses_visited_by_robo_santa(input);

            assert_eq!(result, expected_result);
        }
    }
}
