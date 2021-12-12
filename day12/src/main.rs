use std::char;
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::{Entry, OccupiedEntry};
use std::hash::Hash;
use std::iter::Filter;
use std::slice::Iter;

use itertools::Itertools;

struct Graph {
    nodes: Vec<String>,
    edges: Vec<Option<usize>>,
    big_nodes: HashSet<usize>,
    edge_stride: usize,
    start: usize,
    end: usize,
}

impl Graph {
    fn new(edge_input: Vec<(&str, &str)>) -> Graph {
        let unique_nodes1 = edge_input.iter()
            .map(|(s1, s2)| s1);
        let unique_nodes2 = edge_input.iter()
            .map(|(s1, s2)| s2);
        let edge_stride = unique_nodes1.clone().chain(unique_nodes2.clone())
            .counts().iter().map(|(s, n)| n).max().unwrap().clone();

        let unique = unique_nodes1.chain(unique_nodes2)
            .unique()
            .map(|s| s.to_string())
            .collect_vec();

        let mut mapping: HashMap<String, usize> = HashMap::new();
        unique.iter().enumerate().for_each(|(i, s)| { mapping.insert(s.to_string(), i); });

        // Look for big nodes
        let mut big_nodes: HashSet<usize> = HashSet::new();
        unique.iter().filter(|s| s.chars().next().unwrap().is_uppercase())
            .for_each(|s| { big_nodes.insert(mapping[s]); });

        let start = mapping["start"];
        let end = mapping["end"];

        let edges: Vec<Option<usize>> = (0..unique.len() * edge_stride).map(|i| None).collect_vec();
        let mut graph = Graph {
            nodes: unique,
            big_nodes,
            edges,
            edge_stride,
            start,
            end,
        };

        for edge in edge_input {
            let n1 = mapping.get(edge.0).unwrap();
            let n2 = mapping.get(edge.1).unwrap();
            graph.add_edge(*n1, *n2);
            graph.add_edge(*n2, *n1);
        }

        graph
    }

    /// Adds a directed edge from node1 --> node2
    fn add_edge(&mut self, node1: usize, node2: usize) {
        let free = self.edges[node1 * self.edge_stride..(node1 + 1) * self.edge_stride]
            .iter_mut().find(|n| matches!(n, None));
        *free.unwrap() = Some(node2);
    }

    fn neighbours_of(&self, node: usize) -> &[Option<usize>] {
        &self.edges[node * self.edge_stride..(node + 1) * self.edge_stride]
    }

    fn num_paths(&self, node: usize, visited: &mut HashSet<usize>) -> i32 {
        print!("{},", node);
        if node == self.end {
            return 1;
        }

        if !self.big_nodes.contains(&node) {
            visited.insert(node);
        }

        let mut number_of_paths = 0;
        for neighbour in self.neighbours_of(node) {
            if let Some(neighbour) = neighbour {
                if !visited.contains(neighbour) {
                    number_of_paths += self.num_paths(neighbour.clone(), visited);
                }
            }
        }

        visited.remove(&node);
        // This is a dead end
        return if number_of_paths == 0 {
            0
        } else {
            number_of_paths
        };
    }

    fn num_paths_second_visit(&self, node: usize, visited: &mut HashMap<usize, u8>, path: &mut VecDeque<usize>) -> i32 {
        path.push_back(node);
        if node == self.end {
            println!("{}", path.iter()
                .map(|&i| format!("{}", char::from_digit(i as u32, 16).unwrap()))
                .fold("".to_string(), |a, b| a + &b));
            path.pop_back();
            return 1;
        }

        if !self.big_nodes.contains(&node) {
            match visited.entry(node) {
                Entry::Occupied(mut o) => {*o.get_mut() += 1;}
                Entry::Vacant(mut o) => {visited.insert(node, 1);}
            }
        }

        let mut number_of_paths = 0;
        for neighbour in self.neighbours_of(node) {
            if let Some(neighbour) = neighbour {
                let has_second_visit = visited.values().contains(&2);
                if let Entry::Occupied(entry) = visited.entry(*neighbour) {
                    if *entry.get() == 0 || (*entry.get() < 2 && !has_second_visit) {
                        number_of_paths += self.num_paths_second_visit(neighbour.clone(), visited, path);
                    }
                } else {
                    number_of_paths += self.num_paths_second_visit(neighbour.clone(), visited, path);
                }
            }
        }

        if let Entry::Occupied(mut entry) = visited.entry(node) {
            *entry.get_mut() -= 1;
        }
        path.pop_back();
        // This is a dead end
        return if number_of_paths == 0 {
            0
        } else {
            number_of_paths
        };
    }

    pub fn paths_start_end(&self) -> i32 {
        self.num_paths(self.start, &mut Default::default())
    }

    pub fn paths_second_visit(&self) -> i32 {
        let mut map: HashMap<usize, u8> = HashMap::new();
        // A value != 2 but bigger 2
        map.insert(self.start, 100);
        self.num_paths_second_visit(self.start, &mut map, &mut Default::default())
    }

}

pub fn part1() {
    let content = include_str!("input.txt");
    let edges: Vec<(&str, &str)> = content.lines()
        .map(|l| l.split('-').collect_tuple().unwrap())
        .collect_vec();
    let graph = Graph::new(edges);
    println!("\nNum paths: {}", graph.paths_start_end());
}

pub fn part2() {
    let content = include_str!("input.txt");
    let edges: Vec<(&str, &str)> = content.lines()
        .map(|l| l.split('-').collect_tuple().unwrap())
        .collect_vec();
    let graph = Graph::new(edges);
    println!("\nNum paths: {}", graph.paths_second_visit());
}

fn main() {
    part2();
}
