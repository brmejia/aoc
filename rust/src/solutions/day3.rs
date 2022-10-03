use std::collections::HashSet;

use super::{DaySolution, PartResult};

pub struct Day3 {}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct HousePosition {
    x: isize,
    y: isize,
}

fn houses_visited_by_santa(input: &String) -> usize {
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
    return visited_houses.len();
}

fn houses_visited_by_robo_santa(input: &String) -> usize {
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
    return visited_houses.len();
}

impl DaySolution for Day3 {
    fn get_year(&self) -> usize {
        return 2015;
    }
    fn get_day(&self) -> usize {
        return 3;
    }

    fn part1(&self) -> PartResult {
        let input = self.get_input(None);
        let lines = aoc::parse_input_lines::<String>(input).unwrap();

        return Ok(lines
            .into_iter()
            .map(|line| houses_visited_by_santa(&line).to_string())
            .collect());
    }

    fn part2(&self) -> PartResult {
        let input = self.get_input(None);
        let lines = aoc::parse_input_lines::<String>(input).unwrap();

        return Ok(lines
            .into_iter()
            .map(|line| houses_visited_by_robo_santa(&line).to_string())
            .collect());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_houses_visited_by_santa() {
        let validations = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (input, expected_result) in validations.into_iter() {
            let result = houses_visited_by_santa(&input.to_string());

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_count_houses_visited_by_robo_santa() {
        let validations = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (input, expected_result) in validations.into_iter() {
            let result = houses_visited_by_robo_santa(&input.to_string());

            assert_eq!(result, expected_result);
        }
    }
}
