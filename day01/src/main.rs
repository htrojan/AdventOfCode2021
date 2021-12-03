use itertools::{Itertools, izip};

fn part1() {
    let content = include_str!("input.txt");
    let numbers: Vec<i32> = content.lines().map(|l| l.parse::<i32>().expect("Error while parsing"))
        .collect();

    let collect = numbers.iter().zip(numbers.iter().skip(1))
        .map(|(i1, i2)| i2 - i1)
        .filter(|&i| i > 0).count();
    println!("Measurement increased {} times", collect)
}

fn part2() {
    let content = include_str!("input.txt");
    let numbers: Vec<i32> = content.lines().map(|l| l.parse::<i32>().expect("Error while parsing"))
        .collect();

    let groups = izip!(numbers.iter(), numbers.iter().skip(1), numbers.iter().skip(2))
        .map(|(i1, i2, i3)| i1 + i2 + i3);
    let differences = groups.clone().zip(groups.skip(1))
        .map(|(i1, i2)| i2 - i1)
        .filter(|&i| i > 0).count();
    print!("Measurement of groups increased {} times", differences)

}
fn main() {
    part1();
    part2();
}
