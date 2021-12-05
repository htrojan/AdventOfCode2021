use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, write};
use std::hash::Hash;
use itertools::Itertools;
use crate::LineType::{Diagonal, Horizontal, Vertical};

/// Stores the lines by their position and orientation
#[derive(PartialEq, PartialOrd, Eq, Ord, Copy, Clone)]
enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Copy, Clone)]
struct Line {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Line {
    pub fn from_coordinates(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        Line {
            x1: x1.min(x2),
            x2: x1.max(x2),
            y1: y1.min(y2),
            y2: y1.max(y2),
        }
    }

    pub fn kind(&self) -> LineType {
        if self.x1 != self.x2 && self.y1 == self.y2 {
            LineType::Horizontal
        } else if self.y1 != self.y2 && self.x1 == self.x2 {
            LineType::Vertical
        } else {
            Diagonal
        }
    }

    pub fn intersects(&self, other: &Line) {
        if self.kind() == Horizontal && other.kind() == Vertical {
            if other.x1 >= self.x1 && other.x1 <= self.x2
                && other.y1 >= self.y1 && other.y2 <= self.y2 {

            }
        }
        if self.kind() == Vertical && other.kind() == Horizontal {
            if other.x1 <= self.x1 && other.x2 >= self.x1
                && other.y1 >= self.y1 && other.y1 <= self.y2 {

            }
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

fn part1() {
    let content = include_str!("test.txt");
    let lines = parse(content);
    // Sort by ending position
    let mut horizontal_positions = lines.iter()
        .filter(|l| matches!(l.kind(), Horizontal))
        .sorted_by_key(|l| l.x2).collect_vec();
    let mut vertical_positions = lines.iter()
        .filter(|l| matches!(l.kind(), Vertical))
        .sorted_by_key(|l| l.x1);

    let mut intersections: HashSet<Point> = HashSet::new();
    let mut last_horizontal: usize = 0;

    // Intersections between horizontal/vertical
    while let Some(vertical) = vertical_positions.next() {
        for horizontal in &horizontal_positions[last_horizontal..] {
            if vertical.x1 >= horizontal.x1 && vertical.x1 <= horizontal.x2
                && vertical.y1 <= horizontal.y1 && vertical.y2 >= horizontal.y2 {
                // This vertical line is cut by the last horizontal one
                intersections.insert(Point {x: vertical.x1, y: horizontal.y1});
            } else if horizontal.x1 < vertical.x1{
                // This horizontal is too far away to cut any other line (due to sorting)
                last_horizontal += 1;
            }
        }
    }

    // Intersections between vertical
    let horizontal = lines.iter().filter(|l| matches!(l.kind(), Horizontal))
        .sorted_by(|a, b| {
            Ord::cmp(a.x1, b.x1)
        });
    let mut last_y2 = i32::MIN;
    let mut last_x = i32::MIN;
    for l in vertical {
    }
    println!("Total intersections: {}", intersections.len());
}

fn main() {
    part1();
}
