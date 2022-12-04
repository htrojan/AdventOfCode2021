use std::fmt::Display;
use itertools::Itertools;

// const INPUT: &str = include_str!("testinput.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Self::new(x, y)
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(i32),
    Y(i32),
}

impl Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fold::X(x) => write!(f, "x={}", x),
            Fold::Y(y) => write!(f, "y={}", y),
        }
    }
}

impl Fold {
    fn parse(s: &str) -> Self {
        let mut parts = s.split('=');
        let axis = parts.next().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        if axis.contains('x') {
            Self::X(value)
        } else {
            Self::Y(value)
        }
    }
    fn fold(&self, p: Point) -> Point {
        match self {
            Fold::X(x) => Point::new(Self::fold_along_axis(p.x, *x), p.y),
            Fold::Y(y) => Point::new(p.x, Self::fold_along_axis(p.y, *y)),
        }
    }

    fn fold_along_axis(point_position: i32, fold_position: i32) -> i32 {
        // Fold onto the upper (left) quadrant
        if point_position <= fold_position {
            point_position
        } else {
            let distance = point_position - fold_position;
            let result = point_position - 2 * distance;
            // println!("Folding point {} along {} to {}", point_position, fold_position, result);
            result
        }
    }
}

struct ConsoleDisplay{
    pixels: Vec<bool>,
    width: usize,
    height: usize,
}

impl ConsoleDisplay {
    fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![false; width * height],
            width,
            height,
        }
    }

    fn set(&mut self, x: usize, y: usize, value: bool) {
        self.pixels[y * self.width + x] = value;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.pixels[y * self.width + x]
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.get(x, y) { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn main() {
    let fold_expressions = INPUT.lines().
        skip_while(|line| !line.is_empty())
        .skip(1).map(Fold::parse).collect::<Vec<_>>();
    println!("Fold expressions: {:?}", fold_expressions);

    let first_fold = *fold_expressions.clone().first().unwrap();
    println!("First fold: {}", first_fold);

    let points = INPUT.lines()
        .take_while(|l| !l.is_empty()).map(Point::parse)
        .collect::<Vec<_>>();

    let points_first_fold = points.iter().map(|p| first_fold.fold(*p)).unique().count();
    println!("Points after first fold: {}", points_first_fold);

    let folded = points.into_iter().map(|p| {
        fold_expressions.iter().fold(p, |p, fold| fold.fold(p))
    }).unique().collect::<Vec<_>>();
    println!("Folded points: {:?}", folded.len());

    let xmax = folded.iter().map(|p| p.x).max().unwrap();
    let ymax = folded.iter().map(|p| p.y).max().unwrap();
    let mut display = ConsoleDisplay::new(xmax as usize + 1, ymax as usize + 1);
    for p in folded {
        display.set(p.x as usize, p.y as usize, true);
    }
    display.print();
}
