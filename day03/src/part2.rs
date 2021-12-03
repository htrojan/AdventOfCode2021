const INPUT_LENGTH: usize = 12;

fn parse_input(input: &str) -> Vec<u32> {
    let lines = input.lines();
    let mut result = Vec::<u32>::new();

    for l in lines {
        let numbers = l.as_bytes();
        let mut numeric_number: u32 = 0;

        for i in 0..INPUT_LENGTH {
            // 48 for a '0' and 49 for '1'
            numeric_number += (numbers[i] as u32 - 48) << (INPUT_LENGTH - i - 1);
        }
        result.push(numeric_number);
    }

    return result;
}

fn main() {
    let contents = include_str!("input.txt");
    let numbers = parse_input(&contents);
    let oxygen = count_oxygen_co2(&numbers, false);
    let co2 = count_oxygen_co2(&numbers, true);


    println!("Oxygen: {}", oxygen);
    println!("CO2: {}", co2);
    println!("Result: {}", oxygen * co2);
}

fn count_oxygen_co2(numbers: &Vec<u32>, check_co2: bool) -> u32 {
    let mut valid_index = Vec::<u32>::with_capacity(numbers.len() + 1);
    let mut numbers_left = numbers.len();

    // Fill valid_index with all valid indices
    for n in 0..numbers.len() {
        valid_index.push(n as u32);
    }
    // Indicate end of list
    valid_index.push(u32::MAX);


    // The i-th bit that is currently investigated
    let mut current_bit: usize = INPUT_LENGTH - 1;
    loop {

        // How many 1 have been counted
        let mut count: usize = 0;
        for i in &valid_index {
            if *i == u32::MAX {
                break;
            }

            let number = numbers[*i as usize];
            // Is the i-th bit a 1? Then add a 1. Otherwise add a 0.
            let mut bit = (number >> current_bit) & 1;
            count += bit as usize;
        }

        // Check most-common number (depending on C02 or oxygen check use > or >= to determine the 'most')
        let most_least_common = match check_co2 {
            true => { count * 2 < numbers_left }
            false => { count * 2 >= numbers_left }
        };

        count = 0;
        // Update the valid_index list
        for i in 0..valid_index.len() {
            if valid_index[i] == u32::MAX {
                break;
            }

            let number_index = valid_index[i];
            let number = numbers[number_index as usize];
            // Is the bit the most_common bit?
            let tmp = (number >> current_bit) & 1;
            if tmp == most_least_common as u32 {
                valid_index[count] = valid_index[i];
                count += 1;
            }
        }

        // Update end marker
        valid_index[count] = u32::MAX;
        numbers_left = count;

        if numbers_left <= 1 {
            break;
        }
        current_bit -= 1;
    }
    numbers[valid_index[0] as usize]
}