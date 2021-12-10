use std::fs;

fn calc(data: &Vec<&str>) -> i32 {
    data.iter()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' => {
                        if stack.pop() != Some('(') {
                            return 3;
                        }
                    }
                    ']' => {
                        if stack.pop() != Some('[') {
                            return 57;
                        }
                    }
                    '}' => {
                        if stack.pop() != Some('{') {
                            return 1197;
                        }
                    }
                    '>' => {
                        if stack.pop() != Some('<') {
                            return 25137;
                        }
                    }
                    _ => panic!("unknown char {}", c),
                }
            }
            0
        })
        .sum()
}

fn calc2(data: &Vec<&str>) -> usize {
    let mut scores: Vec<_> = data
        .iter()
        .filter_map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => stack.push(c),
                    ')' => {
                        if stack.pop() != Some('(') {
                            return None;
                        }
                    }
                    ']' => {
                        if stack.pop() != Some('[') {
                            return None;
                        }
                    }
                    '}' => {
                        if stack.pop() != Some('{') {
                            return None;
                        }
                    }
                    '>' => {
                        if stack.pop() != Some('<') {
                            return None;
                        }
                    }
                    _ => panic!("unknown char {}", c),
                }
            }
            Some(stack.iter().rev().fold(0_usize, |score, c| match c {
                '(' => score * 5 + 1,
                '[' => score * 5 + 2,
                '{' => score * 5 + 3,
                '<' => score * 5 + 4,
                _ => panic!("unaccept char in score {}", c),
            }))
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let contnets = fs::read_to_string("input/day10.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    #[test]
    fn part1_test() {
        assert_eq!(26397, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(288957, calc2(&DATA.lines().collect()));
    }
}
