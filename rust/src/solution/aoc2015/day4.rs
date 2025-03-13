use std::{cell::RefCell, collections::HashMap};

use crate::{
    input,
    solution::{PartResult, Solution},
};

#[derive(Debug, Clone)]
pub struct Day4 {
    hashes: RefCell<HashMap<usize, usize>>,
}

impl Day4 {
    pub fn new() -> Self {
        Self {
            hashes: RefCell::new(HashMap::default()),
        }
    }
}

impl Day4 {
    fn find_hash_integer<T>(&self, input: &String, difficulty: T) -> usize
    where
        T: Into<usize> + Copy,
    {
        let difficulty = &difficulty.into();
        if self.hashes.borrow().contains_key(difficulty) {
            return *self.hashes.borrow().get(difficulty).unwrap();
        }

        let max_difficulty = match &self
            .hashes
            .borrow()
            .keys()
            .filter(|&k| k <= difficulty)
            .max()
        {
            &Some(&v) => v,
            None => 1,
        };

        let mut current_lower = match self.hashes.borrow().get(&max_difficulty) {
            Some(&v) => v,
            None => 1,
        };

        let mut seed: String;
        let needle = "0".repeat(*difficulty);
        loop {
            seed = format!("{input}{current_lower}");
            let hash = md5::compute(&seed);

            if format!("{hash:x}").starts_with(&needle) {
                break;
            }
            current_lower += 1;
        }
        self.hashes.borrow_mut().insert(*difficulty, current_lower);

        current_lower
    }
}

impl Solution for Day4 {
    fn part1(&self, input: &str) -> PartResult {
        let base_key = input::parse_input_lines::<String>(input)
            .unwrap()
            .first()
            .expect("Unable to read input")
            .clone();

        let difficulty = 5;
        let k = self.find_hash_integer::<usize>(&base_key, difficulty);
        Ok(vec![k.to_string()])
    }

    fn part2(&self, input: &str) -> PartResult {
        let base_key = input::parse_input_lines::<String>(input)
            .unwrap()
            .first()
            .expect("Unable to read input")
            .clone();
        let difficulty = 6;
        let k = self.find_hash_integer::<usize>(&base_key, difficulty);
        Ok(vec![k.to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_hashes() {
        let validations = vec![("pqrstuv", 5, 1048970)];

        let day = Day4::new();
        for (input, difficulty, expected_result) in validations.into_iter() {
            let k = day.find_hash_integer::<usize>(&input.into(), difficulty);

            assert_eq!(k, expected_result);
        }
    }
}
