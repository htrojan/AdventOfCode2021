use std::collections::HashSet;

use itertools::{Itertools, izip};

struct Translator {
    digits: Vec<HashSet<char>>,
}

impl Translator {

    pub fn from_segment(segment: &Vec<HashSet<char>>) -> Translator {
        Translator {
            digits: Translator::parse_segment(segment)
        }
    }

    pub fn parse_digit(&self, segments: &HashSet<char>) -> i32 {
        let (pos, _) = self.digits.iter()
            .find_position(|number| number.symmetric_difference(&segments).count() == 0)
            .unwrap();
        return pos as i32;
    }

    fn parse_segment(segments: &Vec<HashSet<char>>) -> Vec<HashSet<char>> {
        let one_segment: usize = 2;
        let four_segment: usize = 4;
        let seven_segment: usize = 3;
        let eight_segment: usize = 7;
        // Find unique numbers
        let seg1 = segments.iter().filter(|s| s.len() == one_segment).exactly_one().unwrap();
        let seg4 = segments.iter().filter(|s| s.len() == four_segment).exactly_one().unwrap();
        let seg7 = segments.iter().filter(|s| s.len() == seven_segment).exactly_one().unwrap();
        let seg8 = segments.iter().filter(|s| s.len() == eight_segment).exactly_one().unwrap();

        // Find 'a'
        let a = seg7.iter().filter(|s7| !seg1.contains(s7)).exactly_one().unwrap();
        // println!("a = {}", a);
        // Find '6'
        let seg6 = segments.iter()
            .filter(|s| s.intersection(seg1).count() == 1)
            .filter(|s| s.len() == 6).exactly_one().unwrap();
        // Find 'f'
        let f = seg6.intersection(seg1).exactly_one().unwrap();
        // println!("f = {}", f);
        // Find 'c'
        let c = seg1.iter().filter(|c| *c != f).exactly_one().unwrap();
        // println!("c = {}", c);
        // Find d: 4 without c and f = b and d
        let bd = HashSet::from_iter(seg4.difference(&HashSet::from([*c, *f]))
            .map(|c| *c));
        let seg0 = segments.iter()
            .filter(|seg| seg.len() == 6) // Filter to 0, 6, 9
            .filter(|seg| seg.intersection(&bd).count() == 1).exactly_one().unwrap();// Filter to 0
        let b = seg0.intersection(&bd).exactly_one().unwrap();
        // println!("b = {}", b);
        let d = bd.iter().filter(|c| *c != b).exactly_one().unwrap();
        // println!("d = {}", d);

        let seg2 = segments.iter()
            .filter(|seg| seg.len() == 5)
            .filter(|seg| !seg.contains(f)).exactly_one().unwrap();

        let seg0 = segments.iter()
            .filter(|seg| seg.len() == 6)
            .filter(|seg| !seg.contains(d)).exactly_one().unwrap();
        let seg3 = segments.iter()
            .filter(|seg| seg.len() == 5)
            .filter(|seg| seg.contains(f) && seg.contains(c)).exactly_one().unwrap();
        let seg5 = segments.iter()
            .filter(|seg| seg.len() == 5)
            .filter(|seg| !seg.contains(c)).exactly_one().unwrap();
        let seg8 = segments.iter().find(|seg| seg.len() == 7).unwrap();
        let seg9 = segments.iter()
            .filter(|seg| seg.len() == 6)
            .filter(|seg| seg.contains(c) && seg.contains(d)).exactly_one().unwrap();
        vec![seg0.clone(), seg1.clone(), seg2.clone(), seg3.clone(), seg4.clone(), seg5.clone(),
             seg6.clone(), seg7.clone(), seg8.clone(), seg9.clone()]
    }
}


fn part2() {
    let content = include_str!("input.txt");

    let segments = content.lines()
        .map(|s| s.split('|').next().unwrap().trim())
        .map(|s| s.split_whitespace())
        .map(|s| s.map(|s| HashSet::<char>::from_iter(s.chars())).collect_vec());

    let codes = content.lines()
        .map(|s| s.split(" | ").skip(1).next().unwrap())
        .map(str::trim)
        .map(|s| s.split_whitespace())
        .map(|s| s.map(|s| HashSet::<char>::from_iter(s.chars())).collect_vec());

    let mut total = 0;
    for (segment, code) in izip!(segments, codes) {
        let translator = Translator::from_segment(&segment);
        let digits = code.iter().map(|segments| translator.parse_digit(segments));
        let output = digits.rev().enumerate()
            .map(|(i, d)| 10_i32.pow(i as u32) * d).sum::<i32>();
        total += output;
        println!("Output = {}", output);
    }

    println!("\nTotal output: {}", total);

}

fn part1() {
    let content = include_str!("input.txt");
    // Define number of segments for the uniquely identifiable numbers
    let one_segment: usize = 2;
    let four_segment: usize = 4;
    let seven_segment: usize = 3;
    let eight_segment: usize = 7;

    let digit_occurrences = content.lines()
        .map(|s| s.split(" | ").skip(1).next().unwrap())
        .map(str::trim)
        .map(|s| s.split(' '))
        .flatten()
        .map(|s| s.chars().count())
        .counts();

    let total_unique = digit_occurrences[&one_segment] + digit_occurrences[&four_segment]
        + digit_occurrences[&seven_segment] + digit_occurrences[&eight_segment];
    println!("Found {} ones", digit_occurrences[&one_segment]);
    println!("Found {} fours", digit_occurrences[&four_segment]);
    println!("Found {} sevens", digit_occurrences[&seven_segment]);
    println!("Found {} eights", digit_occurrences[&eight_segment]);
    println!("Found {} unique digits in total", total_unique);
}

fn main() {
    part1();
    part2();
}

