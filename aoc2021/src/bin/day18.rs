use std::{fmt::Display, fs};

#[derive(Debug, Clone)]
enum SnailfishNumber {
    Pair(u32, Vec<SnailfishNumber>),
    Regular(u32),
}

impl SnailfishNumber {
    fn new(s: &str, level: u32) -> Self {
        if s.starts_with('[') {
            let mut parse_level = 0;
            let split_pos = s
                .chars()
                .skip(1)
                .position(|c| match c {
                    '[' => {
                        parse_level += 1;
                        false
                    }
                    ']' => {
                        parse_level -= 1;
                        false
                    }
                    ',' if parse_level == 0 => true,
                    _ => false,
                })
                .unwrap()
                + 1;
            SnailfishNumber::Pair(
                level,
                vec![
                    SnailfishNumber::new(&s[1..split_pos], level + 1),
                    SnailfishNumber::new(&s[split_pos + 1..s.len() - 1], level + 1),
                ],
            )
        } else {
            SnailfishNumber::Regular(s.parse().unwrap())
        }
    }
    fn increase_level(&mut self) {
        match self {
            SnailfishNumber::Pair(level, pair) => {
                *level += 1;
                pair.iter_mut().for_each(|n| n.increase_level())
            }
            SnailfishNumber::Regular(_) => (),
        }
    }
}
impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnailfishNumber::Pair(_, pair) => {
                write!(f, "[{},{}]", pair[0], pair[1])
            }
            SnailfishNumber::Regular(n) => {
                write!(f, "{}", n)
            }
        }
    }
}

fn add(mut left: SnailfishNumber, mut right: SnailfishNumber) -> SnailfishNumber {
    left.increase_level();
    right.increase_level();
    SnailfishNumber::Pair(0, vec![left, right])
}

fn add_leftmost(lefted: &mut u32, num: &mut SnailfishNumber) {
    if *lefted == 0 {
        return;
    }
    match num {
        SnailfishNumber::Pair(_, pair) => {
            add_leftmost(lefted, &mut pair[0]);
        }
        SnailfishNumber::Regular(n) => {
            *n += *lefted;
            *lefted = 0;
        }
    }
}
fn add_rightmost(lefted: &mut u32, num: &mut SnailfishNumber) {
    if *lefted == 0 {
        return;
    }
    match num {
        SnailfishNumber::Pair(_, pair) => {
            add_rightmost(lefted, &mut pair[1]);
        }
        SnailfishNumber::Regular(n) => {
            *n += *lefted;
            *lefted = 0;
        }
    }
}
fn try_explode(num: &mut SnailfishNumber) -> (bool, bool, u32, u32) {
    match num {
        SnailfishNumber::Pair(level, pair) => {
            let (left, right) = pair.split_at_mut(1);
            if *level >= 4 {
                if let SnailfishNumber::Regular(l) = &left[0] {
                    if let SnailfishNumber::Regular(r) = &right[0] {
                        return (true, true, *l, *r);
                    }
                }
            }

            let (just_exploded, exploded, pop_left, mut pop_right) = try_explode(&mut left[0]);
            if just_exploded {
                left[0] = SnailfishNumber::Regular(0);
            }
            if exploded && pop_left == 0 && pop_right == 0 {
                return (false, true, 0, 0);
            }
            add_leftmost(&mut pop_right, &mut right[0]);
            if just_exploded {
                return (false, true, pop_left, pop_right);
            }

            if !exploded {
                let (just_exploded2, exploded2, mut pop_left2, pop_right2) =
                    try_explode(&mut right[0]);
                if just_exploded2 {
                    right[0] = SnailfishNumber::Regular(0);
                }
                if exploded2 && pop_left2 == 0 && pop_right2 == 0 {
                    return (false, true, 0, 0);
                }
                add_rightmost(&mut pop_left2, &mut left[0]);
                return (false, exploded2, pop_left2, pop_right2);
            }
            (false, exploded, pop_left, pop_right)
        }
        _ => (false, false, 0, 0),
    }
}

fn try_split(num: &mut SnailfishNumber) -> bool {
    match num {
        SnailfishNumber::Pair(level, pair) => {
            let splited = match &mut pair[0] {
                SnailfishNumber::Pair(_, _) => try_split(&mut pair[0]),
                SnailfishNumber::Regular(n) => {
                    if *n >= 10 {
                        pair[0] = SnailfishNumber::Pair(
                            *level + 1,
                            vec![
                                SnailfishNumber::Regular(*n / 2),
                                SnailfishNumber::Regular(*n / 2 + *n % 2),
                            ],
                        );
                        true
                    } else {
                        false
                    }
                }
            };
            if splited {
                return true;
            }
            let splited = match &mut pair[1] {
                SnailfishNumber::Pair(_, _) => try_split(&mut pair[1]),
                SnailfishNumber::Regular(n) => {
                    if *n >= 10 {
                        pair[1] = SnailfishNumber::Pair(
                            *level + 1,
                            vec![
                                SnailfishNumber::Regular(*n / 2),
                                SnailfishNumber::Regular(*n / 2 + *n % 2),
                            ],
                        );
                        true
                    } else {
                        false
                    }
                }
            };
            splited
        }
        SnailfishNumber::Regular(_) => false,
    }
}

fn magnitude(num: &SnailfishNumber) -> u32 {
    match num {
        SnailfishNumber::Pair(_, pair) => magnitude(&pair[0]) * 3 + magnitude(&pair[1]) * 2,
        SnailfishNumber::Regular(n) => *n,
    }
}

fn reduce(num: &mut SnailfishNumber) {
    loop {
        let (_, exploded, _, _) = try_explode(num);
        if exploded {
            //println!("after explode: {}", &num);
            continue;
        }
        let splitted = try_split(num);
        if !splitted {
            break;
        }
        //println!("  after split: {}", &num);
    }
}

fn parse(data: &Vec<&str>) -> Vec<SnailfishNumber> {
    let nums: Vec<_> = data
        .iter()
        .map(|line| SnailfishNumber::new(line, 0))
        .collect();
    nums
}

fn calc(data: &Vec<&str>) -> u32 {
    let nums = parse(data);
    let sum = nums
        .into_iter()
        .reduce(|mut sum, item| {
            sum = add(sum, item);
            //println!("    after add: {}", &sum);
            reduce(&mut sum);
            sum
        })
        .unwrap();
    println!("sum: {}", sum);
    magnitude(&sum)
}

fn calc2(data: &Vec<&str>) -> u32 {
    let nums = parse(data);
    let mut max = 0;
    for num_a in nums.iter() {
        for num_b in nums.iter() {
            let mut sum = add(num_a.clone(), num_b.clone());
            reduce(&mut sum);
            max = max.max(magnitude(&sum));
        }
    }
    max
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day18.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    #[test]
    fn part1_test() {
        assert_eq!(4140, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(3993, calc2(&DATA.lines().collect()));
    }
}
