use crate::{
    input,
    solution::{PartResult, Solution},
};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Day8 {}

impl Day8 {
    pub fn new() -> Self {
        Self {}
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

    fn encode_line(input: &str) -> String {
        let encoded_input = input.replace(r#"\"#, r#"\\"#).replace(r#"""#, r#"\""#);
        ["\"", &encoded_input, "\""].join("")
    }

    fn part_1(input: &str) -> usize {
        let input_lines: Vec<String> = input::parse_input_lines(input).unwrap();
        input_lines
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| (s.len(), Day8::count_chars_in_str(s)))
            .fold(0, |acc, x| acc + (x.0 - x.1))
    }
    fn part_2(input: &str) -> usize {
        let input_lines: Vec<String> = input::parse_input_lines(input).unwrap();
        input_lines
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| (s.len(), Day8::encode_line(s).len()))
            .fold(0, |acc, x| acc + (x.1 - x.0))
    }
}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> PartResult {
        let counts = Day8::part_1(input);
        Ok(vec![counts.to_string()])
    }

    fn part2(&self, input: &str) -> PartResult {
        let counts = Day8::part_2(input);
        Ok(vec![counts.to_string()])
    }
}

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::input;

    use super::*;

    const VALIDATIONS: [(&str, &str, usize); 4] = [
        (r#""""#, r#""\"\"""#, 6),
        (r#""abc""#, r#""\"abc\"""#, 5),
        (r#""aaa\"aaa""#, r#""\"aaa\\\"aaa\"""#, 10),
        (r#""\x27""#, r#""\"\\x27\"""#, 6),
    ];

    #[test]
    fn part_2_instructions_encoding() {
        for (input, expected_result, _) in VALIDATIONS.into_iter() {
            let encoded_line = Day8::encode_line(input);
            assert_eq!(encoded_line, expected_result);
        }
    }

    #[test]
    fn part_2_instructions() {
        for (line, expected_encoded_line, _expected_line_count) in VALIDATIONS.iter() {
            let encoded_line = Day8::encode_line(line);
            assert_eq!(encoded_line, *expected_encoded_line);
            assert_eq!(encoded_line.len(), expected_encoded_line.len());
        }

        let input_lines: Vec<_> = VALIDATIONS.iter().map(|l| l.0).collect();
        let expected_result = 19;
        let part_2 = Day8::part_2(input_lines.join("\n").as_str());
        assert_eq!(part_2, expected_result);
    }

    #[test]
    fn part_2_example_file() {
        let test_files_setups = [("../inputs/2015/day8_example.txt", 19)];

        for (input_file_path, _expected_result) in test_files_setups.iter() {
            let file_content = fs::read_to_string(input_file_path).unwrap();

            let part_2 = Day8::part_2(&file_content);
            dbg!(part_2);
        }
    }

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
            let line_result = (input.len(), Day8::count_chars_in_str(input));
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
            ("../inputs/2015/day8_example.txt", 12),
            ("../inputs/2015/day8.txt", 1342),
        ];

        for (input_file_path, expected_result) in test_files_setups.iter() {
            let lines: Vec<String> = input::parse_file_lines(input_file_path).unwrap();

            let mut partial_solutions = Vec::default();
            for line in lines.iter() {
                let line_result = (line.len(), Day8::count_chars_in_str(line));
                partial_solutions.push(line_result);
            }

            let result = partial_solutions.iter().fold(0, |acc, x| acc + (x.0 - x.1));

            assert_eq!(result, expected_result.to_owned());
        }
    }
}
