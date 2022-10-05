use std::collections::HashMap;

use anyhow::Ok;

use super::{DaySolution, PartResult};

pub struct Day4 {
    hashes: HashMap<usize, usize>,
}

impl Day4 {
    pub fn new() -> Self {
        return Self {
            hashes: HashMap::new(),
        };
    }

    fn find_hash_integer<T>(&mut self, input: &String, difficulty: T) -> usize
    where
        T: Into<usize> + Copy,
    {
        let difficulty = &difficulty.into();
        if self.hashes.contains_key(difficulty) {
            return *self.hashes.get(difficulty).unwrap();
        }

        let max_difficulty = match &self.hashes.keys().filter(|&k| k <= difficulty).max() {
            Some(&v) => v,
            None => 1,
        };

        let mut current_lower = match self.hashes.get(&max_difficulty) {
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
        self.hashes.insert(*difficulty, current_lower);

        return current_lower;
    }
}

impl DaySolution for Day4 {
    fn get_year(&self) -> usize {
        return 2015;
    }
    fn get_day(&self) -> usize {
        return 4;
    }

    fn part1(&mut self) -> PartResult {
        let input = self.get_input(None);
        let base_key = aoc::parse_input_lines::<String>(&input)
            .unwrap()
            .first()
            .expect("Unable to read input")
            .clone();

        let difficulty = 5;
        let k = self.find_hash_integer::<usize>(&base_key, difficulty);
        return Ok(vec![k.to_string()]);
    }

    fn part2(&mut self) -> PartResult {
        let input = self.get_input(None);
        let base_key = aoc::parse_input_lines::<String>(&input)
            .unwrap()
            .first()
            .expect("Unable to read input")
            .clone();
        let difficulty = 6;
        let k = self.find_hash_integer::<usize>(&base_key, difficulty);
        return Ok(vec![k.to_string()]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_hashes() {
        let validations = vec![("pqrstuv", 5, 1048970)];

        let mut day = Day4::new();
        for (input, difficulty, expected_result) in validations.into_iter() {
            let k = day.find_hash_integer::<usize>(&input.into(), difficulty);

            assert_eq!(k, expected_result);
        }
    }
}
