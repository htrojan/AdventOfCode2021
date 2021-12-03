enum Direction {
    Forward,
    Down,
    Up
}
fn part1() {
    let content = include_str!("input.txt");
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;

    for l in content.lines() {
        let (direction, increment) = l.split_once(' ').unwrap();
        match direction {
            "forward" => {horizontal += increment.parse::<i32>().expect("Parsing error")},
            "up" => {depth -= increment.parse::<i32>().expect("Parsing error")},
            "down" => {depth += increment.parse::<i32>().expect("Parsing error")},
            &_ => {panic!("This token was not expected!")}
        }
    }

    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);
    println!("Depth * Horizontal: {}", depth * horizontal);
}

fn part2() {
    let content = include_str!("input.txt");
    let mut depth: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut aim: i32 = 0;

    for l in content.lines() {
        let (direction, increment) = l.split_once(' ').unwrap();
        match direction {
            "forward" => {
                let i = increment.parse::<i32>().expect("Parsing error");
                horizontal += i;
                depth += aim * i;
            },
            "up" => {aim -= increment.parse::<i32>().expect("Parsing error")},
            "down" => {aim += increment.parse::<i32>().expect("Parsing error")},
            &_ => {panic!("This token was not expected!")}
        }
    }

    println!("Depth: {}", depth);
    println!("Horizontal: {}", horizontal);
    println!("Aim: {}", aim);
    println!("Depth * Horizontal: {}", depth * horizontal);
}

fn main() {
    println!("Part1: ");
    part1();

    println!("\n\nPart2: ");
    part2();
}
