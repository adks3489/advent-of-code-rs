use parse_display::{Display, FromStr};
use std::fs;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{least}-{most} {letter}: {password}")]
struct PasswordTest {
    least: usize,
    most: usize,
    letter: char,
    password: String,
}

fn main() {
    let contnets = fs::read_to_string("aoc2020/input/day2.txt").unwrap();
    let tests: Vec<PasswordTest> = contnets.lines().map(|line| line.parse().unwrap()).collect();
    let valids = tests.iter().filter(|test| {
        let occurs = test.password.chars().filter(|c| *c == test.letter).count();
        occurs >= test.least && occurs <= test.most
    });
    println!("part1: {}", valids.count());

    let valids = tests.iter().filter(|test| {
        let is_first = test.password.chars().nth(test.least - 1).unwrap() == test.letter;
        let is_second = test.password.chars().nth(test.most - 1).unwrap() == test.letter;
        is_first ^ is_second
    });
    println!("part2: {}", valids.count());
}
