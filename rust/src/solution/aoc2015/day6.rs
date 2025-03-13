use anyhow::Result;
use itertools::Itertools;
use na::DMatrix;
use regex::Regex;
use std::{ops::Add, str::FromStr};

use crate::{
    input,
    solution::{PartResult, Solution},
};

#[derive(Debug, Clone)]
pub struct Day6 {}

impl Day6 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Position(usize, usize);

impl From<(usize, usize)> for Position {
    fn from(t: (usize, usize)) -> Self {
        Self(t.0, t.1)
    }
}

impl FromStr for Position {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [x, y]: [usize; 2] = s
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect_vec()
            .try_into()
            .unwrap();

        Ok(Self(x, y))
    }
}

impl From<Position> for (usize, usize) {
    fn from(val: Position) -> Self {
        (val.0, val.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Toggle,
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmd {
    action: Action,
    position_1: Position,
    position_2: Position,
}

impl FromStr for Action {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "toggle" => Ok(Self::Toggle),
            "turn on" => Ok(Self::On),
            "turn off" => Ok(Self::Off),
            x => panic!("Unknown instruction {}", x),
        }
    }
}

impl FromStr for Cmd {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(?P<action>.+) (?P<pos1>\d+,\d+) .+ (?P<pos2>\d+,\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        let pos1: Position = Position::from_str(captures.name("pos1").unwrap().as_str()).unwrap();
        let pos2: Position = Position::from_str(captures.name("pos2").unwrap().as_str()).unwrap();

        let action: Action = match &captures["action"] {
            "toggle" => Action::Toggle,
            "turn on" => Action::On,
            "turn off" => Action::Off,
            x => panic!("Unknown instruction {}", x),
        };

        Ok(Self {
            action,
            position_1: pos1,
            position_2: pos2,
        })
    }
}

#[derive(Debug)]
struct Grid {
    grid: DMatrix<isize>,
}

impl Grid {
    pub fn new(shape: (usize, usize)) -> Self {
        let grid = DMatrix::zeros(shape.0, shape.1);
        Self { grid }
    }
    fn get_slice_shape(&mut self, pos1: &Position, pos2: &Position) -> (usize, usize) {
        (
            pos1.0.abs_diff(pos2.0).add(1),
            pos1.1.abs_diff(pos2.1).add(1),
        )
    }
    fn set_value(&mut self, pos1: impl Into<Position>, pos2: impl Into<Position>, value: isize) {
        let p1: Position = pos1.into();
        let slice_shape = self.get_slice_shape(&p1, &pos2.into());
        let mut target = self.grid.view_mut(p1.into(), slice_shape);

        for mut row in target.row_iter_mut() {
            row.fill(value);
        }
    }
    fn add_scalar_mut(&mut self, pos1: Position, pos2: Position, value: isize) {
        let slice_shape = self.get_slice_shape(&pos1, &pos2);
        let mut target = self.grid.view_mut(pos1.into(), slice_shape);

        for row in target.row_iter_mut() {
            row.apply_into(|x| *x = 0.max(*x + value));
        }
    }

    pub fn turn_on(&mut self, pos1: impl Into<Position>, pos2: impl Into<Position>) {
        self.set_value(pos1, pos2, 1)
    }

    pub fn turn_off(&mut self, pos1: impl Into<Position>, pos2: impl Into<Position>) {
        self.set_value(pos1, pos2, 0)
    }
    pub fn toggle(&mut self, pos1: Position, pos2: Position) {
        let slice_shape = self.get_slice_shape(&pos1, &pos2);
        let mut target = self.grid.view_mut(pos1.into(), slice_shape);

        target.apply(|x| {
            *x = match x {
                1 => 0,
                0 => 1,
                _x => panic!("Invalid grid value {_x}"),
            }
        });
    }
    pub fn translate_cmd_1(&mut self, cmd: &Cmd) {
        match cmd.action {
            Action::Toggle => self.toggle(cmd.position_1.clone(), cmd.position_2.clone()),
            Action::On => self.turn_on(cmd.position_1.clone(), cmd.position_2.clone()),
            Action::Off => self.turn_off(cmd.position_1.clone(), cmd.position_2.clone()),
        }
    }
    pub fn translate_cmd_2(&mut self, cmd: &Cmd) {
        match cmd.action {
            Action::Toggle => {
                self.add_scalar_mut(cmd.position_1.clone(), cmd.position_2.clone(), 2)
            }
            Action::On => self.add_scalar_mut(cmd.position_1.clone(), cmd.position_2.clone(), 1),
            Action::Off => self.add_scalar_mut(cmd.position_1.clone(), cmd.position_2.clone(), -1),
        }
    }
    pub fn sum(&self) -> isize {
        self.grid.sum()
    }
}

impl Solution for Day6 {
    fn part1(&self, input: &str) -> PartResult {
        let lines = input::parse_input_lines::<String>(input).unwrap();

        let mut grid = Grid::new((1000, 1000));

        lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let cmd = Cmd::from_str(line).unwrap();
                grid.translate_cmd_1(&cmd);
            })
            .collect_vec();

        Ok(vec![grid.sum().to_string()])
    }

    fn part2(&self, input: &str) -> PartResult {
        let lines = input::parse_input_lines::<String>(input).unwrap();

        let mut grid = Grid::new((1000, 1000));

        lines
            .iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let cmd = Cmd::from_str(line).unwrap();
                grid.translate_cmd_2(&cmd);
            })
            .collect_vec();

        Ok(vec![grid.sum().to_string()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instructions_interpretation() {
        let validations = vec![
            (
                "turn on 1,2 through 3,4",
                Cmd {
                    action: Action::On,
                    position_1: Position(1, 2),
                    position_2: Position(3, 4),
                },
            ),
            (
                "turn off 1,2 through 3,4",
                Cmd {
                    action: Action::Off,
                    position_1: Position(1, 2),
                    position_2: Position(3, 4),
                },
            ),
            (
                "toggle 1,2 through 3,4",
                Cmd {
                    action: Action::Toggle,
                    position_1: Position(1, 2),
                    position_2: Position(3, 4),
                },
            ),
        ];

        for (input, expected_result) in validations.into_iter() {
            let result = Cmd::from_str(input).unwrap();

            assert_eq!(result, expected_result);
        }
    }
    #[test]
    fn grid_set_on() {
        let shape = (3, 3);
        let mut grid = Grid::new(shape);
        grid.grid.fill(0);

        grid.turn_on((0, 0), (2, 2));
        assert_eq!(grid.sum() as usize, grid.grid.len())
    }
    #[test]
    fn grid_set_off() {
        let shape = (3, 3);
        let mut grid = Grid::new(shape);
        grid.grid.fill(1);
        grid.turn_off((0, 0), (2, 2));

        assert_eq!(grid.sum(), 0)
    }
    #[test]
    fn grid_toggle() {
        let shape = (3, 3);
        let mut grid = Grid::new(shape);
        grid.grid.fill(1);
        grid.toggle((0, 0).into(), (2, 2).into());
        assert_eq!(grid.sum(), 0);

        let shape = (3, 3);
        let mut grid = Grid::new(shape);
        grid.grid.fill(0);
        grid.toggle((0, 0).into(), (2, 2).into());
        assert_eq!(grid.sum() as usize, grid.grid.len());
    }
    #[test]
    fn test_commands_translation_1() {
        let nrows = 1000;
        let ncols = 1000;
        let mut grid = Grid::new((nrows, ncols));
        let on_cmd =
            Cmd::from_str(format!("turn on 0,0 through {},{}", nrows - 1, ncols - 1).as_str())
                .unwrap();
        let off_cmd =
            Cmd::from_str(format!("turn off 0,0 through {},{}", nrows - 1, ncols - 1).as_str())
                .unwrap();
        let toggle_cmd = Cmd::from_str(
            format!(
                "toggle {},{} through {},{}",
                nrows / 2,
                ncols / 2,
                (nrows / 2) - 1,
                (ncols / 2) - 1
            )
            .as_str(),
        )
        .unwrap();

        // Test ON commands
        grid.translate_cmd_1(&on_cmd);
        assert_eq!(grid.sum() as usize, grid.grid.len());
        grid.translate_cmd_1(&on_cmd);
        assert_eq!(grid.sum() as usize, grid.grid.len());

        // Test OFF commands
        grid.translate_cmd_1(&off_cmd);
        assert_eq!(grid.sum() as usize, 0);
        grid.translate_cmd_1(&off_cmd);
        assert_eq!(grid.sum() as usize, 0);

        // Test TOGGLE commands
        grid.translate_cmd_1(&toggle_cmd);
        assert_eq!(grid.sum() as usize, 4);
        grid.translate_cmd_1(&toggle_cmd);
        assert_eq!(grid.sum() as usize, 0);
    }

    #[test]
    fn test_commands_translation_2() {
        let nrows = 1000;
        let ncols = 1000;
        let mut grid = Grid::new((nrows, ncols));
        let on_cmd = Cmd::from_str("turn on 0,0 through 0,0".to_string().as_str()).unwrap();
        let off_cmd = Cmd::from_str("turn off 0,0 through 0,0".to_string().as_str()).unwrap();
        let toggle_cmd =
            Cmd::from_str(format!("toggle 0,0 through {},{}", nrows - 1, ncols - 1).as_str())
                .unwrap();

        // Test ON commands
        grid.translate_cmd_2(&on_cmd);
        assert_eq!(grid.sum() as usize, 1);
        // Test OFF commands
        grid.translate_cmd_2(&off_cmd);
        assert_eq!(grid.sum() as usize, 0);
        grid.translate_cmd_2(&off_cmd);
        assert_eq!(grid.sum() as usize, 0);
        // Test ON commands
        grid.translate_cmd_2(&on_cmd);
        assert_eq!(grid.sum() as usize, 1);
        // Test OFF commands
        grid.translate_cmd_2(&off_cmd);
        assert_eq!(grid.sum() as usize, 0);
        // Test TOGGLE commands
        grid.translate_cmd_2(&toggle_cmd);
        assert_eq!(grid.sum() as usize, 2000000);
        grid.translate_cmd_2(&toggle_cmd);
        assert_eq!(grid.sum() as usize, 4000000);
    }
}
