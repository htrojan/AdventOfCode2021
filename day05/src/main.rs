use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::vec::IntoIter;
use itertools::Itertools;
use crate::Line::{Diagonal, Horizontal, Vertical};

#[derive(PartialOrd, PartialEq, Hash, Ord, Eq, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

enum Line {
    Horizontal(Point, Point),
    Vertical(Point, Point),
    Diagonal(Point, Point),
}

struct PointIterator {
    line: Line,
    position: i32,
}

impl PointIterator {
    pub fn from(line: Line) -> PointIterator {
        PointIterator {
            line,
            position: -1,
        }
    }
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        match self.line {
            Horizontal(p1, p2) => {
                if (p2.x - p1.x).abs() <= self.position {
                    None
                } else {
                    self.position += 1;
                    Some(Point { x: p1.x.min(p2.x) + self.position, y: p1.y })
                }
            }
            Vertical(p1, p2) => {
                if (p2.y - p1.y).abs() <= self.position {
                    None
                } else {
                    self.position += 1;
                    Some(Point { y: p1.y.min(p2.y) + self.position, x: p1.x })
                }
            }
            Diagonal(p1, p2) => {
                if (p2.x - p1.x).abs() <= self.position {
                    None
                } else {
                    self.position += 1;
                    Some(Point {
                        x: p1.x + (p2.x - p1.x).signum() * self.position,
                        y: p1.y + (p2.y - p1.y).signum() * self.position,
                    })
                }
            }
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = PointIterator;

    fn into_iter(self) -> Self::IntoIter {
        PointIterator::from(self)
    }
}

impl Line {
    pub fn from_coordinates(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        if x1 == x2 {
            Vertical(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
        } else if y1 == y2 {
            Horizontal(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
        } else {
            Diagonal(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
        }
    }

    pub fn from_iterator<I>(mut numbers: I) -> Line
        where I: Iterator<Item=i32> {
        // assert_eq!(numbers.count(), 4);
        Self::from_coordinates(
            numbers.next().unwrap(),
            numbers.next().unwrap(),
            numbers.next().unwrap(),
            numbers.next().unwrap(),
        )
    }
}

fn parse(content: &str) -> Vec<Line> {
    let lines = content.replace(" -> ", ",").lines()
        .map(|s| s.split(','))
        .map(|s| s.map(|c| c.parse::<i32>().expect("Error while parsing int")))
        .map(|n| Line::from_iterator(n)).collect_vec();
    lines
}

fn part2() {
    let content = include_str!("input.txt");
    let lines = parse(content);
    let it = lines.into_iter().map(|l| l.into_iter()).flatten();
    let mut map: HashMap<Point, i32> = HashMap::new();
    for point in it {
        match map.entry(point) {
            Entry::Occupied(mut o) => { *o.get_mut() += 1; }
            Entry::Vacant(v) => { v.insert(0); }
        }
    }
    println!("Total entries: {}", map.values().into_iter().filter(|&&i| i > 0).count());
}

fn part1() {
    let content = include_str!("input.txt");
    let lines = parse(content);
    let it = lines.into_iter()
        .filter(|l| !matches!(l, Diagonal(_, _)))
        .map(|l| l.into_iter()).flatten();
    let mut map: HashMap<Point, i32> = HashMap::new();
    for point in it {
        match map.entry(point) {
            Entry::Occupied(mut o) => { *o.get_mut() += 1; }
            Entry::Vacant(v) => { v.insert(0); }
        }
    }
    println!("Total entries: {}", map.values().into_iter().filter(|&&i| i > 0).count());
}

fn main() {
    part1();
    part2();
}