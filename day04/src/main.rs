use std::fmt::{Display, Formatter};
use ansi_term::Color::Red;
use itertools::{Itertools, repeat_n};
use termcolor::{ColorChoice, StandardStream};

/// Stores information to retrieve all boards that include a certain number
/// BoardLookupTable.iterateBoards(number) returns a slice of all boards
/// with that given number so they can be udpated accordingly
struct BoardLookupTable {
    max_number: usize,
    max_number_occurrence: usize,
    board_table: Vec<u32>,
}

/// Stores the information about what fields are occupied. The 64-bit occupied value is
/// a bitboard with the following layout:
/// 0 0 0 0 0 (Byte 1 - 3, highest bytes)
/// b b b b b - - -  (Byte 4)
/// b b b b b - - -  (Byte 5)
/// b b b b b - - -  (Byte 6)
/// b b b b b - - -  (Byte 7)
/// b b b b b - - -  (Byte 8, lowest)
/// The rest of the u64 are unused zeroes
struct Board {
    occupied_bits: u64,
    numbers: [u8; 25],
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..25 {
            if (i % 5 == 0) && i != 0{
                write!(f, "\n");
            }
            let bit = 1 << ((i / 5) * 8 + (i % 5) + 3);
            let number = self.numbers[i];
            if number < 10 {
                write!(f, " ");
            }
            if self.occupied_bits & bit > 0 {
                write!(f, "{}", Red.paint(number.to_string()));
            } else {
                write!(f, "{}", number);
            }
            write!(f, "  ");
        }
        write!(f, "")
    }
}

impl Board {
    fn occupy_number(&mut self, number: u8) {
        let (pos, _) = self.numbers.iter().find_position(|&&e| e == number).unwrap();
        self.occupied_bits += 1 << ((pos / 5) * 8 + (pos % 5) + 3);
    }

    /// Bitboards: It's a kind of magic
    fn has_won(&self) -> bool {
        // Check horizontal
        let shift1 = self.occupied_bits & (self.occupied_bits << 2);
        let shift2 = shift1 & (shift1 << 1);
        let shift3 = shift2 & (shift2 << 1);

        // Check vertical
        let shift4 = self.occupied_bits & (self.occupied_bits << 2 * 8);
        let shift5 = shift4 & (shift4 << 1 * 8);
        let shift6 = shift5 & (shift5 << 1 * 8);

        shift3 > 0 || shift6 > 0
    }

    fn sum_marked_numbers(&self) -> i32 {
        let mut sum_marked: i32 = 0;
        for i in 0..25 {
            let bit = 1 << ((i / 5) * 8 + (i % 5) + 3);
            if self.occupied_bits & bit > 0 {
                sum_marked += self.numbers[i] as i32;
            }
        }
        sum_marked
    }

    fn sum_unmarked_numbers(&self) -> i32{
        return self.numbers.iter().map(|&i| i as i32).sum::<i32>() - self.sum_marked_numbers();
    }
}

impl BoardLookupTable {
    fn new(max_number: usize, max_number_occurrence: usize) -> BoardLookupTable {
        let board_table = repeat_n(u32::MAX, (max_number + 1) * max_number_occurrence).collect();

        BoardLookupTable {
            max_number,
            max_number_occurrence,
            board_table,
        }
    }

    /// Iterates over all boards (their index respectively) that have the given number
    fn iterate_boards(&self, number: u8) -> &[u32] {
        let start = self.max_number_occurrence * number as usize;
        let end = self.max_number_occurrence * (number as usize + 1);
        &self.board_table[start..end]
    }

    fn add_element(&mut self, board_id: u32, number: u8) {
        let number = number as usize;
        // Find next free space and set the board_id
        for i in 0..self.max_number_occurrence {
            if self.board_table[number * self.max_number_occurrence + i] == u32::MAX {
                self.board_table[number * self.max_number_occurrence + i] = board_id;
                return;
            }
        }
    }
}

fn part1() {
    let input = include_str!("input.txt");
    let (bingo_sequence, search_table, mut boards) = parse_boards(input);


    // Now play bingo :)
    for s in bingo_sequence {
        // Lookup all boards this number has to be added to
        for b in search_table.iterate_boards(s) {
            // Endmarker - could be abstracted in custom iterator
            if *b == u32::MAX {
                break;
            }
            boards[*b as usize].occupy_number(s);
        }

        // Now look for a winner :)
        let mut result = boards.iter().enumerate().filter(|(u, b)| b.has_won());
        // println!("Found {}, results.", result.clone().count());
        println!("Sequence = {}", s);
        println!("{}", boards[1]);
        println!("\n");
        match result.next() {
            None => {}
            Some((u, board)) => {
                println!("Found board {} at sequence {}", u, s);
                println!("Unmarked sum = {}", board.sum_unmarked_numbers());
                println!("Result = {}", board.sum_unmarked_numbers() * s as i32);
                break;
            }
        }
    }
}

fn part2() {
    let input = include_str!("input.txt");
    let (bingo_sequence, search_table, mut boards) = parse_boards(input);
    let board_count = boards.len();


    // Now play bingo :)
    let mut boards_won: u32 = 0;
    for s in bingo_sequence {
        // Lookup all boards this number has to be added to
        for b in search_table.iterate_boards(s) {
            // Endmarker - could be abstracted in custom iterator
            if *b == u32::MAX {
                break;
            }
            let has_already_won = boards[*b as usize].has_won();
            boards[*b as usize].occupy_number(s);
            let has_won_after = boards[*b as usize].has_won();
            boards_won += (has_won_after != has_already_won) as u32;
            if boards_won == board_count as u32 {
                let board = &boards[*b as usize];
                println!("Found board {} at sequence {}", b, s);
                println!("Unmarked sum = {}", board.sum_unmarked_numbers());
                println!("Result = {}", board.sum_unmarked_numbers() * s as i32);
                return;

            }
        }
        println!("Boards won = {}", boards_won);

    }
}

fn parse_boards(input: &str) -> (Vec<u8>, BoardLookupTable, Vec<Board>) {
    let mut lines = input.lines().into_iter();
    let bingo_sequence: Vec<u8> = lines.next().unwrap().trim().split(',')
        .map(|n| n.parse::<u8>().expect("Error while parsing number")).collect();

    println!("Sequence = {}", bingo_sequence.iter()
        .fold(String::new(), |mut a, &b| {
            a.push_str(&*(b.to_string() + " "));
            a
        }));

    let mut numbers = lines.map(|s| s.split_whitespace())
        .flatten()
        .map(|s| s.parse::<u8>().expect("Error while parsing"));

    // Construct search table
    let max_number = bingo_sequence.iter().max().unwrap();
    // Find maximum count of one number
    let max_number_count = numbers.clone().counts().iter().map(|(element, count)| count)
        .max().unwrap().clone();

    let mut search_table = BoardLookupTable::new(*max_number as usize, max_number_count);


    let mut boards: Vec<Board> = Vec::new();
    let mut board_id = 0;

    for board in &numbers.chunks(25) {
        let mut number_array = [0; 25];
        for (i, b) in board.enumerate() {
            search_table.add_element(board_id, b);
            number_array[i] = b;
        }
        boards.push(Board {
            occupied_bits: 0,
            numbers: number_array,
        });
        board_id += 1;
    }
    (bingo_sequence, search_table, boards)
}

fn main() {
    let enabled = ansi_term::enable_ansi_support();
    part1();
    println!();
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_won() {
        let board1 = Board {
            occupied_bits: 0b1111100,
            numbers: [0; 25],
        };
        assert!(board1.has_won());

        let board2 = Board {
            occupied_bits: 0b01111000,
            numbers: [0; 25],
        };
        assert!(!board2.has_won());

        // Vertical
        let board3 = Board {
            occupied_bits: 0b1111000010000000100000001000000010000000,
            numbers: [0; 25],
        };
        assert!(board3.has_won());

        // Vertical
        let board4 = Board {
            occupied_bits: 0b011100001000000010000000100000001000000000000000,
            numbers: [0; 25],
        };
        assert!(!board4.has_won());
    }

    #[test]
    fn test_occupy_number() {
        let numbers1: [u8;25] = [22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19];
        let mut board1 = Board {
            occupied_bits: 0,
            numbers: numbers1
        };
        board1.occupy_number(22);
        board1.occupy_number(13);
        board1.occupy_number(17);
        board1.occupy_number(8);
        board1.occupy_number(4);
        println!("{}", board1.occupied_bits);
        assert_eq!(board1.occupied_bits, 0b00000000_00000000_00000000_01001000_00111000);

    }

    #[test]
    fn test_sum_marked() {
        let input = include_str!("test.txt");
        let (bingo_sequence, search_table, mut boards) = parse_boards(input);
        let numbers1: [u8;25] = [22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19];
        let mut board1 = Board {
            occupied_bits: 0,
            numbers: numbers1
        };

        board1.occupy_number(22);
        board1.occupy_number(13);
        assert_eq!(board1.sum_marked_numbers(), 22 + 13);
        board1.occupy_number(11);
        board1.occupy_number(24);
        assert_eq!(board1.sum_marked_numbers(), 22 + 13 + 11 + 24);

    }

    #[test]
    fn test_construction() {
        let input = include_str!("test.txt");
        let (bingo_sequence, search_table, mut boards) = parse_boards(input);
        let board1: [u8;25] = [22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19];
        let board2: [u8;25] = [3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6];
        let board3: [u8;25] = [14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7];
        assert_eq!(board1, boards[0].numbers);
        assert_eq!(board2, boards[1].numbers);
        assert_eq!(board3, boards[2].numbers);

        let it1 = search_table.iterate_boards(7);
        assert_eq!(it1[0], 0);
        assert_eq!(it1[1], 1);
        assert_eq!(it1[2], 2);
        assert_eq!(it1.get(3), None);

        let it1 = search_table.iterate_boards(11);
        assert_eq!(it1[0], 0);
        assert_eq!(it1[1], 1);
        assert_eq!(it1[2], 2);
        assert_eq!(it1.get(3), None);

        let it1 = search_table.iterate_boards(26);
        assert_eq!(it1[0], 2);
        assert_eq!(it1[1], u32::MAX);
        assert_eq!(it1[2], u32::MAX);
        assert_eq!(it1.get(3), None);

    }
}
