use std::collections::HashSet;
use std::fs;

fn calc(data: &str) -> usize {
    let (mut dots, folding) = parse(data);
    fold_paper(&folding, &mut dots, 1);
    dots.len()
}

fn parse(data: &str) -> (HashSet<(u32, u32)>, Vec<(&str, u32)>) {
    let (dots, folding) = data.split_once("\n\n").unwrap();
    let dots: HashSet<(u32, u32)> = dots.lines().fold(HashSet::new(), |mut map, s| {
        let (x, y) = s.split_once(",").unwrap();
        map.insert((x.parse().unwrap(), y.parse().unwrap()));
        map
    });
    let folding: Vec<(&str, u32)> = folding
        .lines()
        .map(|s| {
            let (direction, pos) = s[11..].split_once("=").unwrap();
            (direction, pos.parse().unwrap())
        })
        .collect();
    (dots, folding)
}

fn calc2(data: &str) {
    let (mut dots, folding) = parse(data);
    fold_paper(&folding, &mut dots, folding.len());
    for y in 0u32..6 {
        for x in 0u32..40 {
            if dots.contains(&(x, y)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}

fn fold_paper(folding: &Vec<(&str, u32)>, dots: &mut HashSet<(u32, u32)>, times: usize) {
    for (direction, pos) in folding[0..times].iter() {
        match *direction {
            "x" => {
                *dots = dots.iter().fold(HashSet::new(), |mut next_dots, dot| {
                    let diff = dot.0 as i32 - *pos as i32;
                    if diff > 0 {
                        next_dots.insert((pos - diff as u32, dot.1));
                    } else {
                        next_dots.insert(*dot);
                    }
                    next_dots
                });
            }
            "y" => {
                *dots = dots.iter().fold(HashSet::new(), |mut next_dots, dot| {
                    let diff = dot.1 as i32 - *pos as i32;
                    if diff > 0 {
                        next_dots.insert((dot.0, pos - diff as u32));
                    } else {
                        next_dots.insert(*dot);
                    }
                    next_dots
                });
            }
            _ => panic!(),
        }
    }
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day13.txt").unwrap();
    let n = calc(&contnets);
    println!("Part1: {}", n);

    calc2(&contnets);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn part1_test() {
        assert_eq!(17, calc(&DATA));
    }
}
