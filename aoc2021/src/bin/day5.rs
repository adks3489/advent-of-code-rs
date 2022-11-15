use std::{collections::HashMap, fs};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate(i32, i32);
#[derive(Debug)]
struct Path {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}
impl Path {
    fn new(s: &str) -> Self {
        let mut s = s.split(" ");
        let mut cood = s.next().unwrap().split(",");
        let (x1, y1) = (
            cood.next().unwrap().parse().unwrap(),
            cood.next().unwrap().parse().unwrap(),
        );
        s.next();
        let mut cood = s.next().unwrap().split(",");
        let (x2, y2) = (
            cood.next().unwrap().parse().unwrap(),
            cood.next().unwrap().parse().unwrap(),
        );
        Path { x1, y1, x2, y2 }
    }
}

fn count_repeated(paths: impl Iterator<Item = Path>) -> usize {
    let mut covered_points = HashMap::new();
    for Path { x1, y1, x2, y2 } in paths {
        if x1 == x2 {
            let r = match y1 > y2 {
                true => (y2..=y1),
                false => (y1..=y2),
            };
            r.for_each(|y| {
                *covered_points.entry((x1, y)).or_insert(0) += 1;
            });
        } else if y1 == y2 {
            let r = match x1 > x2 {
                true => (x2..=x1),
                false => (x1..=x2),
            };
            r.for_each(|x| {
                *covered_points.entry((x, y1)).or_insert(0) += 1;
            });
        } else {
            let r_x = match x1 > x2 {
                true => (x2..=x1).rev().collect::<Vec<_>>(),
                false => (x1..=x2).collect::<Vec<_>>(),
            };
            let r_y = match y1 > y2 {
                true => (y2..=y1).rev().collect::<Vec<_>>(),
                false => (y1..=y2).collect::<Vec<_>>(),
            };
            r_x.iter()
                .zip(r_y)
                .for_each(|(x, y)| *covered_points.entry((*x, y)).or_insert(0) += 1);
        }
    }
    covered_points.iter().filter(|(_, c)| **c >= 2).count()
}

fn calc(data: &Vec<&str>) -> usize {
    let paths = data.iter().filter_map(|line| {
        let p = Path::new(line);
        if p.x1 != p.x2 && p.y1 != p.y2 {
            return None;
        }
        Some(p)
    });
    count_repeated(paths)
}

fn calc2(data: &Vec<&str>) -> usize {
    let paths = data.iter().map(|line| Path::new(line));
    count_repeated(paths)
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day5.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

    #[test]
    fn part1_test() {
        assert_eq!(5, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(12, calc2(&DATA.lines().collect()));
    }
}
