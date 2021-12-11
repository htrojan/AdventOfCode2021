use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

struct Board {
    fields: Vec<u32>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn get_field_at(&self, x: i32, y: i32) -> u32 {
        if x >= self.width as i32 || y >= self.height as i32 || x< 0 || y < 0 {
            u32::MAX
        } else {
            self.fields[y as usize * self.width + x as usize]
        }
    }

    pub fn coords_from_index(&self, index: usize) -> (usize, usize) {
        (index % self.width , index / self.width )
    }

    pub fn from_str(board: &str) -> Board {
        let width = board.lines().next().unwrap().chars().count();
        let height = board.lines().count();
        let fields = board.lines()
            .map(
                |l| l.chars().map(|c| c.to_digit(10).unwrap())
            ).flatten().collect_vec();
        Board {
            fields,
            width,
            height,
        }
    }

    pub fn local_min_iterator(&self) -> LocalMinIterator {
        LocalMinIterator {
            index: 0,
            board: &self,
        }
    }

    pub fn basin_iterator(&self) -> BasinIterator {
        BasinIterator::new(&self)
    }

    pub fn neighbours(&self, x: u32, y: u32) -> NeighbourIterator {
        NeighbourIterator::new(x, y, self.width, self.height)
    }
}

struct NeighbourIterator {
    counter: u8,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl NeighbourIterator {
    pub fn new(x: u32, y: u32, width: usize, height: usize) -> NeighbourIterator {
        NeighbourIterator {
            counter: 0,
            x: x as usize,
            y: y as usize,
            width,
            height
        }
    }
}

impl Iterator for NeighbourIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.counter..4 {
            let result = match self.counter {
                0 => { if self.x > 0 { Some((self.x - 1, self.y)) } else { None } },
                1 => { if self.x < self.width { Some((self.x + 1, self.y)) } else {None} },
                2 => { if self.y > 0 { Some((self.x, self.y - 1)) } else {None} },
                3 => { if self.y < self.height { Some((self.x, self.y + 1)) } else {None} },
                _ => { None }
            };
            if let Some(coord) = result {
                self.counter = i+1;
                return Some(coord);
            }
        }
        None
    }
}

struct LocalMinIterator<'a> {
    index: usize,
    board: &'a Board,
}

struct Basin {
    size: u32
}

struct BasinIterator<'a> {
    board: &'a Board,
    // A set of indices of fields that still has to be searched
    to_search: HashSet<usize>,
}

impl<'a> BasinIterator<'a> {
    fn new(board: &'a Board) -> BasinIterator{
        let mut to_search: HashSet<usize> = HashSet::new();
        (0..board.width*board.height)
            .filter(|&i| board.fields[i] != 9)
            .for_each(|i| {to_search.insert(i);});

        BasinIterator {
            board,
            to_search
        }
    }
}
impl<'a> Iterator for BasinIterator<'a> {
    type Item = Basin;

    fn next(&mut self) -> Option<Self::Item> {
        let search = self.to_search.iter().next().clone();
        if let Some(&start) = search {
            let basin = Basin { size: 0 };
            let mut touched: VecDeque<usize>  = VecDeque::new();
            touched.push_back(start);
            while let Some(search) = touched.pop_front() {
                let (x, y) = self.board.coords_from_index(search);
            }
            None
        } else {
            None
        }
    }
}

struct Minimum {
    x: usize,
    y: usize,
    risk_level: u32,
}

impl<'a> Iterator for LocalMinIterator<'a> {
    type Item = Minimum;

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.index..(self.board.height * self.board.width){
            let (x, y) = self.board.coords_from_index(i);
            let x = x as i32;
            let y = y as i32;
            let value = self.board.get_field_at(x, y);
            let above = self.board.get_field_at(x, y-1);
            let below = self.board.get_field_at(x, y+1);
            let right = self.board.get_field_at(x+1, y);
            let left = self.board.get_field_at(x-1, y);

            if value < above && value < below && value < right && value < left {
                self.index = i + 1;
                let (x, y) = self.board.coords_from_index(i as usize);
                return Some(Minimum {
                    x,
                    y,
                    risk_level: value + 1,
                })
            }
        }

        None
    }
}


fn part1() {
    let content = include_str!("input.txt");
    let board = Board::from_str(content);
    println!("Total risk level: {}", board.local_min_iterator().map(|b| b.risk_level).sum::<u32>());
}

fn part2() {
    let content = include_str!("input.txt");
    let board = Board::from_str(content);
    println!("Total basins found: {}", board.basin_iterator().count());
}

fn main() {
    part1();
    part2();
}
