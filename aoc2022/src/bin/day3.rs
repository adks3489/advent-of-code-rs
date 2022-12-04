use itertools::Itertools;
use std::{collections::HashSet, fs};

fn main() {
    let content = fs::read_to_string("aoc2022/input/day3.txt").unwrap();
    solve1(&content);
    solve2(&content);
}

fn solve1(content: &str) {
    let mut sum = 0;
    content.lines().for_each(|l| {
        let len = l.len() / 2;
        let first = &l[0..len];
        let second = &l[len..l.len()];
        let first: HashSet<char> = first.chars().collect();
        let second: HashSet<char> = second.chars().collect();
        let intersect = first.intersection(&second).nth(0).unwrap();
        let p = get_priority(intersect);
        //println!("{:?} {}", intersect, p);

        sum += p;
    });
    println!("sum of priorities: {}", sum);
}

fn get_priority(item: &char) -> u32 {
    match item.is_ascii_uppercase() {
        true => *item as u32 + 26 - 65 + 1,
        false => *item as u32 - 97 + 1,
    }
}

fn solve2(content: &str) {
    let mut sum = 0;
    for mut chunk in &content.lines().chunks(3) {
        let first: HashSet<char> = chunk.nth(0).unwrap().chars().collect();
        let mut second = HashSet::new();
        for c in chunk.nth(0).unwrap().chars() {
            if first.contains(&c) {
                second.insert(c);
            }
        }
        for c in chunk.nth(0).unwrap().chars() {
            if second.contains(&c) {
                sum += get_priority(&c);
                break;
            }
        }
    }
    println!("sum of priorities: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        solve1(example);
        solve2(example);
    }
}
