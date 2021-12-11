use std::collections::VecDeque;
use crate::Token::Closing;

use crate::Type::{Bracket1, Bracket2, Bracket3, Bracket4};
use crate::ValidationResult::{Incomplete, Invalid, TooMuchClosingBrackets, Valid};

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
enum Type {
    Bracket1,
    Bracket2,
    Bracket3,
    Bracket4,
}

impl Type {
    pub fn score(&self) -> i32{
        match self {
            Bracket1 => {57}
            Bracket2 => {3}
            Bracket3 => {1197}
            Bracket4 => {25137}
        }
    }

    /// The score for part2
    pub fn score2(&self) -> i64{
        match self {
            Bracket1 => {2}
            Bracket2 => {1}
            Bracket3 => {3}
            Bracket4 => {4}
        }
    }
}
#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
enum Token {
    Opening(Type),
    Closing(Type),
    Undefined,
}

impl Token {
    pub fn from_char(c: char) -> Token {
        match c {
            '[' => { Token::Opening(Bracket1) }
            ']' => { Token::Closing(Bracket1) }
            '(' => { Token::Opening(Bracket2) }
            ')' => { Token::Closing(Bracket2) }
            '{' => { Token::Opening(Bracket3) }
            '}' => { Token::Closing(Bracket3) }
            '<' => { Token::Opening(Bracket4) }
            '>' => { Token::Closing(Bracket4) }
            _ => { Token::Undefined }
        }
    }
}

struct TokenStream<I>
    where I: Iterator<Item=char> {
    iterator: I,
}

impl<I> TokenStream<I>
    where I: Iterator<Item=char> {
    fn from_iterator(iterator: I) -> TokenStream<I>
    {
        TokenStream {
            iterator
        }
    }
}

impl<I> Iterator for TokenStream<I>
    where I: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.iterator.next() {
            Some(Token::from_char(c))
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
struct ValidationError {
    expected: Token,
    reality: Token
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
enum ValidationResult {
    Valid,
    Invalid(ValidationError),
    TooMuchClosingBrackets,
    Incomplete(VecDeque<Type>),
    EncounteredUndefinedToken
}

fn validate<I>(mut tokens: TokenStream<I>) -> ValidationResult
    where I: Iterator<Item=char> {
    let mut stack: VecDeque<Type> = VecDeque::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Opening(t) => {
                stack.push_front(t);
            }
            Token::Closing(t) => {
                if let Some(pop) = stack.pop_front() {
                    if pop != t {
                        return Invalid(ValidationError {
                            expected: Closing(pop),
                            reality: Closing(t)
                        });
                    }
                } else {
                    return TooMuchClosingBrackets;
                }
            }
            Token::Undefined => {
                return ValidationResult::EncounteredUndefinedToken;
            }
        }
    }

    if stack.is_empty() {
        Valid
    } else {
        Incomplete(stack)
    }
}

fn part1_and_part2() {
    let content = include_str!("input.txt");
    let mut points_part1 = 0;
    let mut incomplete_scores: Vec<i64> = Vec::new();

    for line in content.lines() {
        let stream = TokenStream::from_iterator(line.chars());
        let valid = validate(stream);
        println!("Valid: {:?}", valid);
        match valid {
            Invalid(error) => {
                if let Closing(t) = error.reality {
                   points_part1 += t.score();
                }
            }
            Incomplete(stack) => {
                let mut points: i64 = 0;
                for t in stack {
                    points *= 5;
                    points += t.score2();
                }
                incomplete_scores.push(points);
            }
            _ => {}
        }
    }

    println!("Part1: Total score: {}", points_part1);

    println!("Number of incomplete lines: {}", incomplete_scores.len());
    incomplete_scores.sort();
    let middle_score = incomplete_scores[incomplete_scores.len()/2];
    println!("Part2: Total score: {}", middle_score);
}

fn main() {
    part1_and_part2();
}

