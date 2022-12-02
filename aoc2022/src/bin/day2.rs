use std::fs;

fn main() {
    let contnets = fs::read_to_string("aoc2022/input/day2.txt").unwrap();
    let mut score = 0;
    let mut score2 = 0;
    contnets.lines().for_each(|l| {
        if l.is_empty() {
            return;
        }
        let oppo = l.chars().nth(0).unwrap();
        let me = l.chars().nth(2).unwrap();
        score += score_part1(oppo, me);
        score2 += score_part2(oppo, me);
    });
    println!("score: {} {}", score, score2);
}

fn score_part1(oppo: char, me: char) -> i32 {
    match oppo {
        'A' => match me {
            'X' => 1 + 3,
            'Y' => 2 + 6,
            'Z' => 3 + 0,
            _ => panic!(),
        },
        'B' => match me {
            'X' => 1 + 0,
            'Y' => 2 + 3,
            'Z' => 3 + 6,
            _ => panic!(),
        },
        'C' => match me {
            'X' => 1 + 6,
            'Y' => 2 + 0,
            'Z' => 3 + 3,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn score_part2(oppo: char, me: char) -> i32 {
    match oppo {
        'A' => match me {
            'X' => 3 + 0,
            'Y' => 1 + 3,
            'Z' => 2 + 6,
            _ => panic!(),
        },
        'B' => match me {
            'X' => 1 + 0,
            'Y' => 2 + 3,
            'Z' => 3 + 6,
            _ => panic!(),
        },
        'C' => match me {
            'X' => 2 + 0,
            'Y' => 3 + 3,
            'Z' => 1 + 6,
            _ => panic!(),
        },
        _ => panic!(),
    }
}
