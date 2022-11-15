use std::{collections::HashSet, fs};

fn is_movable(
    map: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    x_len: usize,
    y_len: usize,
    current: char,
) -> bool {
    match map[y][x] {
        '>' => current == 'v',
        'v' => false,
        _ => true,
    }
}

fn calc(data: &Vec<&str>) -> i32 {
    let mut map: Vec<Vec<char>> = data.iter().map(|line| line.chars().collect()).collect();
    let y_len = map.len();
    let x_len = map[0].len();
    let mut step = 0;
    loop {
        step += 1;
        let mut next_map = map.clone();
        let mut move_count = 0;
        let mut moved_set = HashSet::new();
        for y in 0..y_len {
            for x in 0..x_len {
                match map[y][x] {
                    '>' => {
                        let next_x = if x == x_len - 1 { 0 } else { x + 1 };
                        if map[y][next_x] == '.' {
                            next_map[y][next_x] = '>';
                            next_map[y][x] = '.';
                            moved_set.insert((y, x, next_x));
                            move_count += 1;
                        } else {
                            next_map[y][x] = '>';
                        }
                    }
                    _ => (),
                }
            }
        }
        moved_set.into_iter().for_each(|(y, x, next_x)| {
            map[y][x] = '.';
            map[y][next_x] = '>';
        });
        for y in 0..y_len {
            for x in 0..x_len {
                match map[y][x] {
                    'v' => {
                        let next_y = if y == y_len - 1 { 0 } else { y + 1 };
                        if map[next_y][x] == '.' {
                            next_map[next_y][x] = 'v';
                            next_map[y][x] = '.';
                            move_count += 1;
                        } else {
                            next_map[y][x] = 'v';
                        }
                    }
                    _ => (),
                }
            }
        }
        if move_count == 0 {
            break;
        }
        map = next_map;
    }
    step
}

fn calc2(data: &Vec<&str>) -> i32 {
    0
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day25.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

    #[test]
    fn part1_test() {
        assert_eq!(58, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(0, calc2(&DATA.lines().collect()));
    }
}
