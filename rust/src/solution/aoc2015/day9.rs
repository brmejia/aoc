#![allow(dead_code)]

use itertools::Itertools;

use crate::input;
use crate::solution::error::Result;
use crate::solution::{PartResult, Solution};
use hex;
use sha2::{Digest, Sha256};

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Day9 {
    input_hash: RefCell<Option<String>>,
    graph: RefCell<Option<AdjacencyList>>,
}

impl Day9 {
    pub fn new() -> Self {
        Self {
            input_hash: RefCell::new(None),
            graph: RefCell::new(None),
        }
    }

    fn hash_input(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }
    fn build_graph(&self, input: &str) -> Result<AdjacencyList> {
        let input_hash = self.hash_input(input);
        if let Some(current_hash) = self.input_hash.borrow().clone() {
            if input_hash == current_hash {
                if let Some(g) = self.graph.borrow_mut().clone() {
                    debug!("Returning graph from cache");
                    return Ok(g);
                }
            }
        }

        debug!("Preparing a new graph for traversal");
        let lines: Vec<String> = input::parse_input_lines(input).unwrap();

        let mut graph = AdjacencyList::default();
        for line in lines.iter() {
            let edge = parsing::try_parse_edge(line).unwrap();
            graph.try_add_edge(edge).unwrap();
        }

        self.graph.replace(Some(graph.clone()));
        self.input_hash.replace(Some(input_hash));
        Ok(graph)
    }
    fn search_full_paths(&self, input: &str) -> Result<Vec<(Vec<Vertex>, u32)>> {
        let graph = self.build_graph(input)?;
        let paths = search_full_paths_on_graph(&graph);
        Ok(paths)
    }
}

impl Solution for Day9 {
    fn part1(&self, input: &str) -> PartResult {
        let paths = self.search_full_paths(input).unwrap();
        let sorted_paths = paths.into_iter().sorted_by_key(|p| p.1).collect_vec();
        if let Some(shortest_path) = sorted_paths.first() {
            let ppath = shortest_path.0.iter().map(|v| v.0.clone()).join(" -> ");
            debug!("[{}] {}", shortest_path.1, ppath);
            Ok(vec![format!("{}", shortest_path.1)])
        } else {
            Ok(vec!["Incomplete".to_string()])
        }
    }

    fn part2(&self, input: &str) -> PartResult {
        let paths = self.search_full_paths(input).unwrap();
        let sorted_paths = paths.into_iter().sorted_by_key(|p| p.1).collect_vec();

        if let Some(longest_path) = sorted_paths.last() {
            let ppath = longest_path.0.iter().map(|v| v.0.clone()).join(" -> ");
            debug!("[{}] {}", longest_path.1, ppath);
            Ok(vec![format!("{}", longest_path.1)])
        } else {
            Ok(vec!["Incomplete".to_string()])
        }
    }
}

type VertexId = u8;
type EdgeValue = u16;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd)]
struct Vertex(String);

impl Vertex {
    pub fn new(name: &str) -> Self {
        Self(name.into())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
struct Edge {
    left: Vertex,
    right: Vertex,
    value: EdgeValue,
}

impl Edge {
    pub fn new(from: &str, to: &str, value: EdgeValue) -> Self {
        Self {
            left: Vertex::new(from),
            right: Vertex::new(to),
            value,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct AdjacencyList {
    inner: HashMap<Vertex, HashMap<Vertex, EdgeValue>>,
    vertexes: HashSet<Vertex>,
}

impl AdjacencyList {
    pub fn try_add_edge(&mut self, edge: Edge) -> Result<()> {
        self.vertexes.insert(edge.left.clone());
        self.vertexes.insert(edge.right.clone());

        match self.get_mut(&edge.left) {
            Some(connections) => {
                connections.insert(edge.right.clone(), edge.value);
            }
            None => {
                self.insert(
                    edge.left.clone(),
                    HashMap::from([(edge.right.clone(), edge.value)]),
                );
            }
        };
        match self.get_mut(&edge.right) {
            Some(connections) => {
                connections.insert(edge.left.clone(), edge.value);
            }
            None => {
                self.insert(edge.right, HashMap::from([(edge.left, edge.value)]));
            }
        };
        Ok(())
    }
}

impl Deref for AdjacencyList {
    type Target = HashMap<Vertex, HashMap<Vertex, EdgeValue>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AdjacencyList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

mod parsing {
    use nom::branch::alt;
    use nom::sequence::separated_pair;
    use nom::{
        Parser,
        bytes::complete::tag,
        character::complete::{self, alphanumeric1, space0, space1},
        sequence::delimited,
    };

    use crate::error::Result;
    use crate::input::error::InputError;

    use super::Edge;
    fn vertex_name(input: &str) -> nom::IResult<&str, &str> {
        alphanumeric1(input)
    }

    fn distance_separator(input: &str) -> nom::IResult<&str, &str> {
        alt((
            delimited(space1, tag("to"), space1),
            delimited(space0, alt((tag("=>"), tag("->"))), space0),
        ))
        .parse(input)
    }

    pub fn distance_parser(input: &str) -> nom::IResult<&str, (&str, &str, u16)> {
        let (remainder, (src, dst)) =
            separated_pair(vertex_name, distance_separator, vertex_name).parse(input)?;
        let (remainder, _) = delimited(space0, tag("="), space0).parse(remainder)?;
        let (remainder, value) = complete::u16(remainder)?;

        Ok((remainder, (src, dst, value)))
    }

    pub fn try_parse_edge(input_line: &str) -> Result<Edge> {
        let (_, (src, dst, value)) =
            distance_parser(input_line).map_err(|e| InputError::Parsing(format!("{}", e)))?;

        Ok(Edge::new(src, dst, value))
    }
}

fn traverse_graph(
    graph: &AdjacencyList,
    current: &Vertex,
    path: &mut Vec<Vertex>,
    visited: &mut HashMap<Vertex, bool>,
    paths: &mut Vec<(Vec<Vertex>, u32)>,
    mut acc: u32,
) -> bool {
    debug!("Visiting {}", current.0);
    if let Some(&current_visited) = visited.get(current) {
        if current_visited {
            debug!("{} Already visited", current.0);
            return false;
        }
    }
    // Pre
    path.push(current.clone());
    visited.insert(current.clone(), true);

    let ppath = path.iter().map(|v| v.0.clone()).join(" -> ");
    debug!("[{}] {}", acc, ppath);

    let all_visited: bool = visited.values().all(|&v| v);
    if all_visited {
        paths.push((path.clone(), acc));
        debug!("Walked all vertexes. Current path was stored");
        path.pop();
        visited.insert(current.clone(), false);
        return true;
    }

    // Recourse
    if let Some(current_edges) = graph.get(current) {
        for (next, edge_value) in current_edges {
            acc += u32::from(*edge_value);
            traverse_graph(graph, next, path, visited, paths, acc);
            acc -= u32::from(*edge_value);
        }
    }

    // Post
    path.pop();
    visited.insert(current.clone(), false);

    false
}

fn search_full_paths_on_graph(graph: &AdjacencyList) -> Vec<(Vec<Vertex>, u32)> {
    let mut paths: Vec<(Vec<Vertex>, u32)> = Vec::default();

    for start_vertex in graph.vertexes.iter() {
        // debug!("\n{:?} ----------------------------------", &start_vertex);

        let mut visited: HashMap<Vertex, bool> =
            HashMap::from_iter(graph.vertexes.iter().map(|v| (v.clone(), false)));
        // dbg!(&visited);

        let mut path: Vec<Vertex> = Vec::default();
        // dbg!(&path);
        traverse_graph(graph, start_vertex, &mut path, &mut visited, &mut paths, 0);
        // dbg!(&paths);
    }

    paths
}

#[cfg(test)]
mod tests {

    use crate::input;

    use super::*;
    use tracing_subscriber::{EnvFilter, fmt, prelude::*};

    #[test]
    fn test_distance_parser() {
        let validations = vec![
            ("Faerun to Norrath = 129", ("Faerun", "Norrath", 129)),
            ("Faerun to Tristram = 58", ("Faerun", "Tristram", 58)),
            ("Faerun ->A =42", ("Faerun", "A", 42)),
            ("Faerun=>B = 42", ("Faerun", "B", 42)),
        ];

        for (input, expected_result) in validations.into_iter() {
            let (_, result) = parsing::distance_parser(input).unwrap();
            assert_eq!(result, expected_result);
            dbg!(result);
        }
    }

    #[test]
    fn test_edge_parsing() {
        let validations = vec![
            (
                "Faerun to Norrath = 129",
                Edge::new("Faerun", "Norrath", 129),
            ),
            (
                "Faerun to Tristram = 58",
                Edge::new("Faerun", "Tristram", 58),
            ),
            (
                "Faerun to Tristram = 42",
                Edge::new("Faerun", "Tristram", 42),
            ),
        ];

        let mut graph = AdjacencyList::default();
        for (input, expected_result) in validations.into_iter() {
            let result = parsing::try_parse_edge(input).unwrap();
            assert_eq!(result, expected_result);
            graph.try_add_edge(result).unwrap();
        }
    }

    #[test]
    fn test_part_1a() {
        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env())
            .init();

        let examples = [(
            vec![
                "London to Dublin = 464",
                "London to Belfast = 518",
                "Dublin to Belfast = 141",
            ],
            605,
        )];

        for (k, example) in examples.iter().enumerate() {
            debug!("\nExample {k} ========================================================");

            // Build adjacency list
            let mut graph = AdjacencyList::default();
            for input_line in example.0.iter() {
                let edge = parsing::try_parse_edge(input_line).unwrap();
                graph.try_add_edge(edge).unwrap();
            }

            let paths = search_full_paths_on_graph(&graph);

            debug!("\n**************************************************");
            for (p, res) in paths.iter() {
                let ppath = p.iter().map(|v| v.0.clone()).join(" -> ");
                debug!("[{}] {}", res, ppath);
            }
            debug!("**************************************************");
            let shortest_path = paths.iter().min_by_key(|&p| p.1).unwrap();
            let ppath = shortest_path.0.iter().map(|v| v.0.clone()).join(" -> ");
            debug!("[{}] {}", shortest_path.1, ppath);
            debug!("**************************************************");
            assert_eq!(shortest_path.1, example.1);
        }
        panic!()
    }

    #[test]
    fn test_part_1_from_file() {
        let test_files_setups = [
            ("../inputs/2015/day9.txt", 207),
            ("../inputs/2015/day9_jkpr.txt", 251),
        ];

        for (input_file_path, expected_result) in test_files_setups.iter() {
            let lines: Vec<String> = input::parse_file_lines(input_file_path).unwrap();

            let mut graph = AdjacencyList::default();
            for line in lines.iter() {
                let edge = parsing::try_parse_edge(line).unwrap();
                graph.try_add_edge(edge).unwrap();
            }
            let paths = search_full_paths_on_graph(&graph);

            for (p, _res) in paths.iter() {
                let ppath = p.iter().map(|v| v.0.clone()).join(" -> ");
                debug!("{}", ppath);
            }
            let shortest_path = paths.iter().min_by_key(|&p| p.1).unwrap();

            dbg!(shortest_path);
            assert_eq!(shortest_path.1, *expected_result);
        }
    }
}
