use itertools::Itertools;

fn part1_part2(simulation_days: i32) {
    let content = include_str!("input.txt");
    let mut population: [u64; 9] = [0; 9];
    content.split(',').map(|s| s.parse::<i32>().expect("Failed parsing number"))
        .for_each(|i| population[i as usize] += 1);

    for day in 0..simulation_days {
        print!("Day {}: ", day);
        println!("{}", population.iter().fold(String::new(), |a, b| a + " " +b.to_string().as_str()));
        let babies = population[0];
        (0..8).for_each(|i| population[i] = population[i+1]);
        population[8] = babies;
        population[6] += babies;
    }
    println!("Population: {}", population.into_iter().sum::<u64>());
}


fn main() {
    println!("Part 1: ");
    part1_part2(80);
    println!("\n\nPart 2: ");
    part1_part2(256);
}
