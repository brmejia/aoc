use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use regex::Regex;
use std::str::FromStr;
use strum_macros::EnumString;

use crate::solutions::{get_problem_input, Day, DaySolution, PartResult, Solution};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Input {
    Value(u16),
    Signal(String),
}

#[derive(Debug, PartialEq, Eq, EnumString)]
enum Operation {
    #[strum(disabled)]
    Wire,
    #[strum(serialize = "NOT")]
    Not,
    #[strum(serialize = "AND")]
    And,
    #[strum(serialize = "OR")]
    Or,
    #[strum(serialize = "LSHIFT")]
    LShift(u16),
    #[strum(serialize = "RSHIFT")]
    RShift(u16),
}

#[derive(Debug, PartialEq, Eq)]
struct Connection {
    operation: Operation,
    inputs: HashSet<Input>,
    output: String,
}
impl Connection {}

impl FromStr for Connection {
    type Err = regex::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"^(?P<in1>[a-z0-9]+)?\s?(?P<action>[A-Z]+)?\s?(?P<in2>[a-z0-9]+)? -> (?P<output>\D+)",
        )
        .unwrap();
        let captures = re
            .captures(s)
            .expect("Unable to parse connection from input {s}. Wrong input format.");

        let mut inputs: HashMap<&str, Input> = ["in1", "in2"]
            .iter()
            .filter_map(|&input_name| match captures.name(input_name) {
                Some(iv) => match iv.as_str().parse::<u16>().ok() {
                    Some(val) => Some((input_name, Input::Value(val))),
                    None => Some((input_name, Input::Signal(iv.as_str().to_string()))),
                },
                None => None,
            })
            .collect();

        let operation = match captures.name("action") {
            Some(action) => {
                let op = Operation::from_str(action.as_str()).unwrap();
                match op {
                    Operation::LShift(_) | Operation::RShift(_) => {
                        let in2 = inputs.remove("in2").unwrap();
                        match (&op, in2) {
                            (Operation::LShift(_), Input::Value(val)) => {
                                Operation::LShift(val as u16)
                            }
                            (Operation::RShift(_), Input::Value(val)) => {
                                Operation::RShift(val as u16)
                            }
                            (Operation::RShift(_) | Operation::LShift(_), Input::Signal(_)) => {
                                panic!(
                                "Input 2 must be parseable as u16 if action is LShift or RShift."
                            )
                            }
                            _ => op,
                        }
                    }
                    op => op,
                }
            }
            None => match inputs.get("in1").unwrap() {
                Input::Value(_) => Operation::Wire,
                Input::Signal(_) => Operation::Wire,
            },
        };

        let output = captures.name("output").unwrap().as_str().to_string();
        Ok(Connection {
            operation,
            inputs: inputs.into_values().collect(),
            output,
        })
    }
}

#[derive(Debug)]
struct Circuit {
    pub signals: HashMap<String, u16>,
    pub connections: Vec<Connection>,
    pub remaining_connections: Vec<Connection>,
}

impl Circuit {
    pub fn new(signals: HashMap<String, u16>, connections: Vec<Connection>) -> Self {
        Self {
            signals,
            connections,
            remaining_connections: Vec::new(),
        }
    }
    fn get_connection_missing_inputs(&self, conn: &Connection) -> Option<HashSet<String>> {
        let missing_inputs = conn
            .inputs
            .iter()
            .filter_map(|input| match input {
                Input::Value(_) => None,
                Input::Signal(signal_name) => match !self.signals.contains_key(signal_name) {
                    true => Some(signal_name.clone()),
                    false => None,
                },
            })
            .collect::<HashSet<String>>();
        match missing_inputs.len() {
            0 => None,
            _ => Some(missing_inputs),
        }
    }

    fn get_input_value(&self, input: &Input) -> Option<u16> {
        match input {
            Input::Value(v) => Some(*v),
            Input::Signal(signal_name) => self.signals.get(signal_name).copied(),
        }
    }

    fn process_wire(&mut self, connection: &Connection) {
        let input = connection.inputs.iter().next().unwrap();
        match self.get_input_value(input) {
            Some(value) => {
                self.signals.insert(connection.output.clone(), value);
            }
            None => panic!("Input {:?} is not defined", input),
        }
    }

    fn process_not(&mut self, connection: &Connection) {
        let input = connection.inputs.iter().next().unwrap();
        match self.get_input_value(input) {
            Some(value) => {
                self.signals.insert(connection.output.clone(), !value);
            }
            None => panic!("Input {:?} is not defined", input),
        }
    }
    fn process_and(&mut self, connection: &Connection) {
        if self.get_connection_missing_inputs(connection).is_some() {
            return;
        }
        let value = connection.inputs.iter().fold(u16::MAX, |accu, input| {
            accu & self.get_input_value(input).unwrap()
        });

        self.signals.insert(connection.output.clone(), value);
    }
    fn process_or(&mut self, connection: &Connection) {
        if self.get_connection_missing_inputs(connection).is_some() {
            return;
        }
        let value = connection.inputs.iter().fold(u16::MIN, |accu, input| {
            accu | self.get_input_value(input).unwrap()
        });

        self.signals.insert(connection.output.clone(), value);
    }

    fn process_shift(&mut self, connection: &Connection) {
        if self.get_connection_missing_inputs(connection).is_some() {
            return;
        }
        let input = connection.inputs.iter().next().unwrap();
        let input_value = self.get_input_value(input).unwrap();
        let value = match connection.operation {
            Operation::LShift(shift_count) => input_value << shift_count,
            Operation::RShift(shift_count) => input_value >> shift_count,
            _ => panic!(
                "Unable to process shift operation. Received {:?}",
                connection.operation
            ),
        };

        self.signals.insert(connection.output.clone(), value);
    }

    fn process_connection(&mut self, connection: Connection) {
        match connection.operation {
            Operation::Wire => self.process_wire(&connection),
            Operation::Not => self.process_not(&connection),
            Operation::And => self.process_and(&connection),
            Operation::Or => self.process_or(&connection),
            Operation::LShift(_) | Operation::RShift(_) => self.process_shift(&connection),
        };
        self.connections.push(connection)
    }

    fn process_remaining_connections(&mut self) {
        let mut i = 0;
        while i < self.remaining_connections.len() {
            if self
                .get_connection_missing_inputs(&self.remaining_connections[i])
                .is_some()
            {
                i += 1;
                continue;
            }
            let conn = self.remaining_connections.remove(i);

            self.process_connection(conn);
            i = 0
        }
    }

    fn add_connection(&mut self, connection: Connection) {
        let missing_input_names = self.get_connection_missing_inputs(&connection);
        match missing_input_names {
            None => self.process_connection(connection),
            Some(_missing_names) => {
                self.remaining_connections.push(connection);
                return;
            }
        }

        self.process_remaining_connections();
    }
}

impl Default for Circuit {
    fn default() -> Self {
        Self::new(HashMap::default(), Vec::default())
    }
}

#[derive(Debug)]
pub struct Day7 {
    pub year: u16,
    pub day: u8,
    circuit: Circuit,
    input_lines: Vec<String>,
}

impl Day7 {
    pub fn new() -> Self {
        let (year, day) = (2015, 7);

        let input = get_problem_input(year, day).unwrap();
        Self {
            year,
            day,
            circuit: Circuit::default(),
            input_lines: aoc::parse_input_lines(&input).unwrap(),
        }
    }
}

impl Day for Day7 {
    fn get_year(&self) -> usize {
        self.year.into()
    }
    fn get_day(&self) -> usize {
        self.day.into()
    }
}

impl Solution for Day7 {
    fn part1(&mut self) -> PartResult {
        for line in self.input_lines.iter() {
            let conn = Connection::from_str(line).unwrap();
            self.circuit.add_connection(conn);
        }

        Ok(vec![self.circuit.signals.get("a").unwrap().to_string()])
    }

    fn part2(&mut self) -> PartResult {
        let prev_value = self.circuit.signals.get("a").unwrap().to_owned();
        // Clear all signal values and redefine the value for b
        self.circuit.signals.clear();
        self.circuit.signals.insert("b".to_string(), prev_value);

        // ensure there are no remaining connections
        self.circuit.remaining_connections.clear();

        // Move all circuit connections to remaining_connections excluding signal b
        let mut remaining_connections = self
            .circuit
            .connections
            .drain(..)
            .filter(|conn| conn.output != *"b")
            .collect::<Vec<_>>();
        self.circuit
            .remaining_connections
            .append(&mut remaining_connections);

        self.circuit.process_remaining_connections();

        let result = self.circuit.signals.get("a").unwrap().to_owned();
        Ok(vec![result.to_string()])
    }
}

impl DaySolution for Day7 {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire() {
        let mut circuit = Circuit::default();

        let connections = ["42 -> x", "x -> y"].map(|input| Connection::from_str(input).unwrap());

        for conn in connections.into_iter() {
            circuit.add_connection(conn);
        }
        assert_eq!(*circuit.signals.get("x").unwrap(), 42u16);
        assert_eq!(*circuit.signals.get("y").unwrap(), 42u16);
    }
    #[test]
    fn test_not() {
        let mut circuit = Circuit::default();

        let connections =
            ["42 -> x", "NOT x -> y"].map(|input| Connection::from_str(input).unwrap());

        for conn in connections.into_iter() {
            circuit.add_connection(conn);
        }
        assert_eq!(*circuit.signals.get("x").unwrap(), 42u16);
        assert_eq!(*circuit.signals.get("y").unwrap(), u16::MAX - 42);
    }

    #[test]
    fn test_and() {
        let mut circuit = Circuit::default();

        let connections = ["42 -> x", "11 -> y", "x AND y -> z"]
            .map(|input| Connection::from_str(input).unwrap());

        for conn in connections.into_iter() {
            circuit.add_connection(conn);
        }
        assert_eq!(*circuit.signals.get("x").unwrap(), 42u16);
        assert_eq!(*circuit.signals.get("y").unwrap(), 11u16);
        assert_eq!(*circuit.signals.get("z").unwrap(), 42u16 & 11u16);
    }

    #[test]
    fn test_circuit() {
        let mut circuit = Circuit::default();

        let connections = [
            "NOT z -> nz",
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> f",
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
            "x AND y -> z",
        ]
        .map(|input| Connection::from_str(input).unwrap());

        for conn in connections.into_iter() {
            circuit.add_connection(conn);
        }
        dbg!(&circuit.signals);
        assert_eq!(*circuit.signals.get("nz").unwrap(), !72);
        assert_eq!(*circuit.signals.get("z").unwrap(), 72);
        assert_eq!(*circuit.signals.get("d").unwrap(), 72);
        assert_eq!(*circuit.signals.get("e").unwrap(), 507);
        assert_eq!(*circuit.signals.get("f").unwrap(), 492);
        assert_eq!(*circuit.signals.get("g").unwrap(), 114);
        assert_eq!(*circuit.signals.get("h").unwrap(), 65412);
        assert_eq!(*circuit.signals.get("i").unwrap(), 65079);
        assert_eq!(*circuit.signals.get("x").unwrap(), 123);
        assert_eq!(*circuit.signals.get("y").unwrap(), 456);
    }

    #[test]
    fn test_examples() {
        let validations: Vec<(&str, Connection)> = vec![
            (
                "123 -> x",
                Connection {
                    operation: Operation::Wire,
                    inputs: HashSet::from([Input::Value(123)]),
                    output: "x".into(),
                },
            ),
            (
                "456 -> y",
                Connection {
                    operation: Operation::Wire,
                    inputs: HashSet::from([Input::Value(456)]),
                    output: "y".into(),
                },
            ),
            (
                "ab -> ly",
                Connection {
                    operation: Operation::Wire,
                    inputs: HashSet::from([Input::Signal("ab".into())]),
                    output: "ly".into(),
                },
            ),
            (
                "x AND y -> d",
                Connection {
                    operation: Operation::And,
                    inputs: HashSet::from([Input::Signal("x".into()), Input::Signal("y".into())]),
                    output: "d".into(),
                },
            ),
            (
                "x OR y -> e",
                Connection {
                    operation: Operation::Or,
                    inputs: HashSet::from([Input::Signal("x".into()), Input::Signal("y".into())]),
                    output: "e".into(),
                },
            ),
            (
                "x LSHIFT 2 -> f",
                Connection {
                    operation: Operation::LShift(2),
                    inputs: HashSet::from([Input::Signal("x".into())]),
                    output: "f".into(),
                },
            ),
            (
                "y RSHIFT 2 -> g",
                Connection {
                    operation: Operation::RShift(2),
                    inputs: HashSet::from([Input::Signal("y".into())]),
                    output: "g".into(),
                },
            ),
            (
                "NOT y -> i",
                Connection {
                    operation: Operation::Not,
                    inputs: HashSet::from([Input::Signal("y".into())]),
                    output: "i".into(),
                },
            ),
        ];

        for (input, expected_result) in validations.into_iter() {
            let connection = Connection::from_str(input).unwrap();
            assert_eq!(connection, expected_result);
        }
    }
}
