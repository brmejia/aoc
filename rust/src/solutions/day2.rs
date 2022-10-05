use std::{fmt::Display, str::FromStr};

use super::{DaySolution, PartResult};

pub struct Day2 {}

#[derive(Debug)]
struct Package {
    l: usize,
    w: usize,
    h: usize,
}

impl Package {
    fn new(l: usize, w: usize, h: usize) -> Self {
        return Self { l, w, h };
    }

    fn required_surface(&self) -> usize {
        let [l, w, h] = [self.l, self.w, self.h];
        return 2 * (l * w + l * h + w * h) + (l * w).min(l * h).min(w * h);
    }
    fn required_ribon(&self) -> usize {
        let [l, w, h] = [self.l, self.w, self.h];
        return (l * w * h) + (2 * (l + w + h - l.max(w).max(h)));
    }
}

impl<T: FromStr + Display> From<T> for Package {
    fn from(string_like: T) -> Self {
        let [l, w, h]: [usize; 3] = aoc::split_line::<usize>(&string_like.to_string(), 'x')
            .unwrap()
            .try_into()
            .expect(format!("Error parsing package: {}", string_like).as_str());

        Package::new(l, w, h)
    }
}

impl DaySolution for Day2 {
    fn get_year(&self) -> usize {
        return 2015;
    }
    fn get_day(&self) -> usize {
        return 2;
    }

    fn part1(&mut self) -> PartResult {
        let input = self.get_input(None);
        let lines = aoc::parse_input_lines::<String>(&input).unwrap();

        let packages: Vec<Package> = lines.into_iter().map(|line| Package::from(line)).collect();

        let total_surface: usize = packages
            .iter()
            .map(|package| package.required_surface())
            .sum();

        return Ok(vec![total_surface.to_string()]);
    }

    fn part2(&mut self) -> PartResult {
        let input = self.get_input(None);
        let lines = aoc::parse_input_lines::<String>(&input).unwrap();

        let packages: Vec<Package> = lines.into_iter().map(|line| Package::from(line)).collect();

        let total_ribon: usize = packages
            .iter()
            .map(|package| package.required_ribon())
            .sum();

        return Ok(vec![total_ribon.to_string()]);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn required_package_surface() {
        let validations = vec![("2x3x4".to_string(), 58), ("1x1x10".to_string(), 43)];

        for (dimensions, expected) in validations.into_iter() {
            let package = Package::from(dimensions);

            assert_eq!(package.required_surface(), expected);
        }
    }

    #[test]
    fn required_package_ribon() {
        let validations = vec![("2x3x4".to_string(), 34), ("1x1x10".to_string(), 14)];

        for (dimensions, expected) in validations.into_iter() {
            let package = Package::from(dimensions);

            assert_eq!(package.required_ribon(), expected);
        }
    }
}
