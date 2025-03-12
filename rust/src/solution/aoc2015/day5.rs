use crate::{
    input,
    solution::{PartResult, Solution},
};
use itertools::Itertools;

#[derive(Debug)]
pub struct Day5 {}

impl Day5 {
    pub fn new() -> Self {
        Self {}
    }
}
fn count_vowels(input: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    input.chars().filter(|c| vowels.contains(c)).count()
}

fn have_consecutive_chars(input: &str) -> bool {
    input.chars().tuple_windows().any(|(a, b)| a == b)
}

fn have_forbidden_pattern(input: &str) -> bool {
    let forbidden_patterns = ["ab", "cd", "pq", "xy"];
    forbidden_patterns.iter().any(|pat| input.contains(pat))
}

fn is_nice(input: &str) -> bool {
    (count_vowels(input) >= 3) & have_consecutive_chars(input) & !have_forbidden_pattern(input)
}

fn super_nice_rule_1(input: &str) -> bool {
    input.chars().tuple_windows::<(char, char)>().any(|pat| {
        input
            .matches(format!("{}{}", pat.0, pat.1).as_str())
            .count()
            >= 2
    })
}

fn super_nice_rule_2(input: &str) -> bool {
    input.chars().tuple_windows().any(|(a, _, b)| a == b)
}

fn is_super_nice(input: &str) -> bool {
    super_nice_rule_1(input) & super_nice_rule_2(input)
}

impl Solution for Day5 {
    fn part1(&mut self, input: &str) -> PartResult {
        let lines = input::parse_input_lines::<String>(input).unwrap();

        let nice_words: usize = lines.iter().filter(|line| is_nice(line)).count();

        Ok(vec![nice_words.to_string()])
    }

    fn part2(&mut self, input: &str) -> PartResult {
        let lines = input::parse_input_lines::<String>(input).unwrap();

        let super_nice_words: usize = lines.iter().filter(|line| is_super_nice(line)).count();

        Ok(vec![super_nice_words.to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vowels_count() {
        let validations = vec![
            ("", 0),
            ("aaa", 3),
            ("a", 1),
            (" g ", 0),
            (" o ", 1),
            ("", 0),
        ];

        for (input, expected_result) in validations.into_iter() {
            let result = count_vowels(input);

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_consecutive_chars() {
        let validations = vec![
            ("11", true),
            ("222", true),
            ("aabbcc", true),
            ("  g ", true),
            ("a", false),
            (" g ", false),
            (" o ", false),
            ("", false),
        ];

        for (input, expected_result) in validations.into_iter() {
            let result = have_consecutive_chars(input);

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_forbiden_patterns() {
        let validations = vec![
            ("1ab1", true),
            ("cdcdcdcd", true),
            ("abxy", true),
            ("xasdy", false),
            ("123", false),
        ];

        for (input, expected_result) in validations.into_iter() {
            let result = have_forbidden_pattern(input);

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_nice_strings() {
        let validations = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for (input, expected_result) in validations.into_iter() {
            let result = is_nice(input);

            assert_eq!(result, expected_result);
        }
    }

    #[test]
    fn test_super_nice_strings() {
        let validations = vec![
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
        ];

        for (input, expected_result) in validations.into_iter() {
            let result = is_super_nice(input);

            assert_eq!(result, expected_result);
        }
    }
}
