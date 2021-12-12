use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::path::Iter;

use itertools::Itertools;

struct Board {
    fields: Vec<u32>,
    width: usize,
    height: usize,
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rows = self.fields
            .iter()
            .map(|c| char::from_digit(*c, 10).unwrap())
            .chunks(self.width);
        for r in &rows {
            writeln!(f, "{}", r.format(""));
        }
        write!(f, "")
    }
}

impl Board {
    pub fn from_str(str: &str) -> Board {
        let width = str.lines().next().unwrap().chars().count();
        let height = str.lines().count();
        let fields = str.chars()
            .filter_map(|c| c.to_digit(10))
            .collect_vec();
        assert_eq!(fields.len(), width * height);
        Board {
            width,
            height,
            fields,
        }
    }

    pub fn get_at_coords(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            Some(self.fields[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_neighbours_of(&self, index: usize) -> NeighbourIterator {
        Index { width: self.width, height: self.height }.get_neighbours_of(index)
    }

    pub fn get_index(&self) -> Index {
        Index { width: self.width, height: self.height }
    }
}

#[derive(Copy, Clone)]
struct Index {
    width: usize,
    height: usize,
}

impl Index {
    pub fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    pub fn coords_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_neighbours_of(self, index: usize) -> NeighbourIterator {
        let (x, y) = self.index_to_coords(index);
        NeighbourIterator {
            x,
            y,
            counter: 0,
            board: self,
        }
    }
}

struct NeighbourIterator {
    x: usize,
    y: usize,
    board: Index,
    counter: u8,
}

impl Iterator for NeighbourIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.counter..8 {
            let result = match i {
                // horizontal, vertical
                0 => { if self.x > 0 { Some((self.x - 1, self.y)) } else { None } }
                1 => { if self.x < self.board.width - 1 { Some((self.x + 1, self.y)) } else { None } }
                2 => { if self.y > 0 { Some((self.x, self.y - 1)) } else { None } }
                3 => { if self.y < self.board.height - 1 { Some((self.x, self.y + 1)) } else { None } }
                // Diagonal
                4 => { if self.x > 0 && self.y > 0 { Some((self.x - 1, self.y - 1)) } else { None } }
                5 => {
                    if self.x < self.board.width - 1 && self.y < self.board.width - 1
                    { Some((self.x + 1, self.y + 1)) } else { None }
                }
                6 => { if self.x > 0 && self.y < self.board.height - 1 { Some((self.x - 1, self.y + 1)) } else { None } }
                7 => { if self.y > 0 && self.x < self.board.width - 1 { Some((self.x + 1, self.y - 1)) } else { None } }
                _ => { None }
            };
            if let Some(coord) = result {
                self.counter = i + 1;
                return Some(coord);
            }
        }
        None
    }
}

fn part2() {
    let content = include_str!("input.txt");
    let mut board = Board::from_str(content);
    let mut flashing: VecDeque<usize> = VecDeque::new();
    let mut flashing_octopus = 0;
    println!("Before:\n{}", board);

    let mut step = 1;
    // Simulate steps
    while board.fields.iter().any(|&f| f > 0) {
        board.fields.iter_mut().for_each(|i| *i += 1);
        board.fields.iter().enumerate().filter(|(index, &i)| i > 9)
            .for_each(|(index, i)| flashing.push_back(index));
        flashing_octopus += flashing.len();


        while let Some(position) = flashing.pop_front() {
            let index = board.get_index();
            // println!("Position: {}", position);
            for n in board.get_neighbours_of(position)
                .map(|(x, y)| index.coords_to_index(x, y)) {
                // println!("n: {}", n);
                board.fields[n] += 1;
                // Flashes for the first time as it reaches level greater 9
                if board.fields[n] == 10 {
                    flashing.push_back(n);
                    flashing_octopus += 1;
                }
            }
        }
        board.fields.iter_mut().filter(|i| *i > &mut 9)
            .for_each(|i| *i = 0);
        let all_flashed = !board.fields.iter().any(|&f| f > 0);
        println!("\nAfter Step {}:\n{}",step, board);
        step += 1;
    }


}
fn part1() {
    let content = include_str!("input.txt");
    let mut board = Board::from_str(content);
    let mut flashing: VecDeque<usize> = VecDeque::new();
    let mut flashing_octopus = 0;
    println!("Before:\n{}", board);

    // Simulate steps
    for i in 0..100 {
        board.fields.iter_mut().for_each(|i| *i += 1);
        board.fields.iter().enumerate().filter(|(index, &i)| i > 9)
            .for_each(|(index, i)| flashing.push_back(index));
        flashing_octopus += flashing.len();


        while let Some(position) = flashing.pop_front() {
            let index = board.get_index();
            // println!("Position: {}", position);
            for n in board.get_neighbours_of(position)
                .map(|(x, y)| index.coords_to_index(x, y)) {
                // println!("n: {}", n);
                board.fields[n] += 1;
                // Flashes for the first time as it reaches level greater 9
                if board.fields[n] == 10 {
                    flashing.push_back(n);
                    flashing_octopus += 1;
                    // println!("Position {} flashed!", n);
                }
            }
        }
        board.fields.iter_mut().filter(|i| *i > &mut 9)
            .for_each(|i| *i = 0);
        println!("\nAfter Step {}:\n{}", i+1, board);
    }

    println!("Total flashing octopus: {}", flashing_octopus);
}

fn main() {
    part1();
    part2();
}
