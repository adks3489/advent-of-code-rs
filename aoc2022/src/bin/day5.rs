use parse_display::{Display, FromStr};
use std::fs;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("move {count} from {from} to {to}")]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let content = fs::read_to_string("aoc2022/input/day5.txt").unwrap();
    let content = content[content.find("\n\n").unwrap() + 2..].to_owned();
    let stacks = vec![
        vec!['B', 'V', 'S', 'N', 'T', 'C', 'H', 'Q'],
        vec!['W', 'D', 'B', 'G'],
        vec!['F', 'W', 'R', 'T', 'S', 'Q', 'B'],
        vec!['L', 'G', 'W', 'S', 'Z', 'J', 'D', 'N'],
        vec!['M', 'P', 'D', 'V', 'F'],
        vec!['F', 'W', 'J'],
        vec!['L', 'N', 'Q', 'B', 'J', 'V'],
        vec!['G', 'T', 'R', 'C', 'J', 'Q', 'S', 'N'],
        vec!['J', 'S', 'Q', 'C', 'W', 'D', 'M'],
    ];
    solve1(&content, &stacks);
    solve2(&content, &stacks);
}

fn solve1(content: &str, stacks: &Vec<Vec<char>>) {
    let mut stacks = stacks.clone();
    content.lines().for_each(|l| {
        let ins = l.parse::<Instruction>().unwrap();
        for _ in 0..ins.count {
            let n = stacks[ins.from - 1].pop().unwrap();
            stacks[ins.to - 1].push(n);
        }
        //println!("{:?}", stacks);
    });
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!("");
}

fn solve2(content: &str, stacks: &Vec<Vec<char>>) {
    let mut stacks = stacks.clone();
    content.lines().for_each(|l| {
        let ins = l.parse::<Instruction>().unwrap();
        let mut temp = Vec::new();
        for _ in 0..ins.count {
            let n = stacks[ins.from - 1].pop().unwrap();
            temp.push(n);
        }
        for _ in 0..ins.count {
            let n = temp.pop().unwrap();
            stacks[ins.to - 1].push(n);
        }
        //println!("{:?}", stacks);
    });
    for s in stacks {
        print!("{}", s.last().unwrap());
    }
    println!("");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let example = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        solve1(example, &stacks);
        solve2(example, &stacks);
    }
}
