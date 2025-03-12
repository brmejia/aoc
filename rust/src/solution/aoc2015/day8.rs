use std::collections::VecDeque;

use crate::{
    input,
    solution::{PartResult, Solution},
};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct Day8 {
    input_lines: Vec<String>,
}

impl Day8 {
    pub fn new() -> Self {
        Self {
            input_lines: vec![],
        }
    }

    fn with_input(&mut self, input: &str) -> &Self {
        self.input_lines = input::parse_input_lines(input).unwrap();
        self
    }

    fn count_and_replace_hex(input: &str) -> (usize, String) {
        lazy_static! {
            // static ref HEX_RE: Regex = Regex::new(r"\\x\d{2}").unwrap();
            static ref HEX_RE: Regex = Regex::new(r"\\x[a-fA-F0-9]{2}").unwrap();
        }
        let count = HEX_RE.captures_iter(input).count();
        (count, HEX_RE.replace_all(input, "").to_string())
    }

    fn count_and_replace_quotes(input: &str) -> (usize, String) {
        lazy_static! {
            static ref START_DQUOTE_RE: Regex = Regex::new(r#"^["]"#).unwrap();
            static ref END_DQUOTE_RE: Regex = Regex::new(r#"["]$"#).unwrap();
            static ref DQUOTE_RE: Regex = Regex::new(r#"\\""#).unwrap();
        }
        let _start = START_DQUOTE_RE.captures_iter(input).count();
        let input = START_DQUOTE_RE.replace_all(input, "").to_string();

        let _end = END_DQUOTE_RE.captures_iter(&input).count();
        let input = END_DQUOTE_RE.replace_all(&input, "").to_string();

        let quotes_count = DQUOTE_RE.captures_iter(&input).count();
        let input = DQUOTE_RE.replace_all(&input, "").to_string();

        (quotes_count, input)
    }
    fn count_and_replace_backslashes(input: &str) -> (usize, String) {
        // lazy_static! {
        //     static ref BSLASH_RE: Regex = Regex::new(r#"\\"#).unwrap();
        // }
        // let count = BSLASH_RE
        //     .captures_iter(input)
        //     // .map(|c| {
        //     //     dbg!(&c);
        //     //     dbg!(&c.len());
        //     //     c.len()
        //     // })
        //     .count();
        // let input = BSLASH_RE.replace_all(input, "").to_string();

        let count = input.matches(r"\\").count();
        // dbg!(count);
        let input = input.replacen(r"\\", r"", count);

        (count, input)
    }

    pub fn count_chars_in_str(input: &str) -> usize {
        let (bslashes_count, remainder) = Self::count_and_replace_backslashes(input);
        let (hex_count, remainder) = Self::count_and_replace_hex(&remainder);
        let (quotes_count, remainder) = Self::count_and_replace_quotes(&remainder);

        hex_count + quotes_count + bslashes_count + remainder.len()
    }

}

impl Solution for Day8 {
    fn part1(&mut self, input: &str) -> PartResult {
        self.with_input(input);
        let counts = self.input_lines
            .iter()
            .map(|s| (s.len(), Day8::count_chars_in_str(s) ) )
            .fold(0, |acc, x| acc + (x.0 - x.1));

        Ok(vec![counts.to_string()])
    }

    fn part2(&mut self, input: &str) -> PartResult {
        self.with_input(input);
        Ok(vec!["Incomplete".to_string()])
    }
}

enum EscapedState {
    None,
    Quote,
    Hex(isize),
}

pub fn part_a(input: &str) -> usize {
    let mut count = 0;
    let mut count_b = 0;
    for line in input.trim().split('\n') {
        let mut chars: VecDeque<char> = line.chars().collect();
        count_b += chars.len();

        chars.pop_back();
        chars.pop_front();

        let mut escaped_state = EscapedState::None;
        for c in &chars {
            match escaped_state {
                EscapedState::None => match c {
                    '\\' => {
                        escaped_state = EscapedState::Quote;
                        count += 1;
                    }
                    _ => {
                        count += 1;
                    }
                },
                EscapedState::Quote => match c {
                    '\\' | '"' => {
                        escaped_state = EscapedState::None;
                    }
                    'x' => {
                        escaped_state = EscapedState::Hex(2);
                    }
                    _ => {
                        eprintln!("{line} {c}");
                        panic!();
                    }
                },
                EscapedState::Hex(i) => {
                    if i == 1 {
                        escaped_state = EscapedState::None;
                    } else {
                        escaped_state = EscapedState::Hex(i - 1);
                    }
                }
            }
        }
    }
    count_b - count
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::input;

    use super::*;

    #[test]
    fn test_part_1_examples() {
        let mut validations = vec![
            (r#""""#, (2, 0)),
            (r#""\""#, (3, 1)),
            (r#""\x27""#, (6, 1)),
            (r#""\"\"""#, (6, 2)),
            (r#""abc""#, (5, 3)),
            (r#""aaa\"aaa""#, (10, 7)),
            (r#""\x27""#, (6, 1)),
            (r#""\x4f\x22""#, (10, 2)),
            (r#""\xab\"\xab""#, (12, 3)),
            (r#""ikfv""#, (6, 4)),
            (r#""\xd2cuho""#, (10, 5)),
            (r#""vj""#, (4, 2)),
            (r#""d""#, (3, 1)),
            (r#""\\g""#, (5, 2)),
            (r#""ubgxxcvnltzaucrzg\\xcez""#, (25, 22)),
        ];

        let mut partial_solutions = Vec::default();
        for (input, expected_result) in validations.iter_mut() {
            println!("==========================================================");
            dbg!(&expected_result);
            // dbg!(&input);
            let line_result = (input.len(), Day8::count_chars_in_str(input) );
            dbg!(&line_result);
            assert_eq!(line_result, expected_result.to_owned());

            partial_solutions.push(line_result);
        }
    }

    #[test]
    fn test_part_1_from_file() {
        let test_files_setups = [
            ("../inputs/2015/day8_jkpr.txt", 1350),
            ("../inputs/2015/day8_jocelyn_stericher.txt", 1371),
            ("../inputs/2015/day8_example.txt", 17),
            ("../inputs/2015/day8.txt", 1342),
        ];

        for (input_file_path, expected_result) in test_files_setups.iter() {
            let file_content = fs::read_to_string(input_file_path).unwrap();
            let ref_response = part_a(&file_content);
            let lines: Vec<String> = input::parse_file_lines(input_file_path).unwrap();

            let mut partial_solutions = Vec::default();
            for line in lines.iter() {
                let line_result = (line.len(), Day8::count_chars_in_str(line) );
                let ref_line_result = part_a(line);

                if ref_line_result != (line_result.0 - line_result.1) {
                    dbg!(line, ref_line_result, line_result);
                }
                partial_solutions.push(line_result);
            }

            let result = partial_solutions.iter().fold(0, |acc, x| acc + (x.0 - x.1));
            assert_eq!(ref_response, result);

            assert_eq!(result, expected_result.to_owned());
        }
    }
}
