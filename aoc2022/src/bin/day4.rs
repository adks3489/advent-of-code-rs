use parse_display::{Display, FromStr};
use std::fs;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{start1}-{end1},{start2}-{end2}")]
struct RangePair {
    start1: i32,
    end1: i32,
    start2: i32,
    end2: i32,
}

fn main() {
    let content = fs::read_to_string("aoc2022/input/day4.txt").unwrap();
    solve1(&content);
    solve2(&content);
}

fn solve1(content: &str) {
    let mut sum = 0;
    content.lines().for_each(|l| {
        let pair: RangePair = l.parse::<RangePair>().unwrap();
        if pair.start1 <= pair.start2 && pair.end1 >= pair.end2 {
            sum += 1;
        } else if pair.start2 <= pair.start1 && pair.end2 >= pair.end1 {
            sum += 1;
        }
    });
    println!("fully contain: {}", sum);
}

fn solve2(content: &str) {
    let mut sum = 0;
    content.lines().for_each(|l| {
        let pair: RangePair = l.parse::<RangePair>().unwrap();
        if pair.start1 <= pair.end2 && pair.start2 <= pair.end1 {
            sum += 1;
        }
    });
    println!("overlap: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let example = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        solve1(example);
        solve2(example);
    }
}
