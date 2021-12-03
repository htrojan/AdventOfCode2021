use std::fs;

fn main() {
    const INPUT_LENGTH: usize = 12;

    let contents = include_str!("input.txt");
    let lines = contents.lines();
    let mut counts: [u32; INPUT_LENGTH] = [0; INPUT_LENGTH];
    let mut linecount = 0;

    for l in lines {
        // 48 for a '0' and 49 for '1'
        let numbers = l.as_bytes();
        for i in 0..INPUT_LENGTH {
            counts[i] += (numbers[i] - 48) as u32;
        }

        linecount += 1;
    }

    println!("{:?}", counts);
    println!("Total linecount: {}", linecount);
    let mut gamma: u32 = 0;

    let mut invert_mask: u32 = 0;
    for i in 0..INPUT_LENGTH {
        invert_mask += 1 << i;
    }

    println!("Invert mask: {}", invert_mask);

    for i in 0..INPUT_LENGTH {
        if counts[i] > linecount / 2 {
            gamma += 1 << INPUT_LENGTH - 1 - i;
            println!("i = {}, Plus {}", i, 1 << INPUT_LENGTH - 1 - i);
            // print!("1");
        }
    }
    println!();
    let epsilon = !gamma & invert_mask;
    println!("Resulting (decimal) gamma: {}", gamma);
    println!("Resulting (decimal) epsilon: {}", epsilon);
    println!("Resulting power consumption: {}", gamma * epsilon)
}
