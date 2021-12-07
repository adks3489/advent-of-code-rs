use std::fs;

fn calc(data: &Vec<&str>) -> usize {
    let pos: Vec<_> = data.iter().map(|n| n.parse::<i32>().unwrap()).collect();
    let min = *pos.iter().min().unwrap();
    let max = *pos.iter().max().unwrap();
    (min..=max)
        .map(|i| pos.iter().map(|p| ((p - i).abs() as usize)).sum())
        .min()
        .unwrap()
}

fn calc2(data: &Vec<&str>) -> usize {
    let pos: Vec<_> = data.iter().map(|n| n.parse::<i32>().unwrap()).collect();
    let min = *pos.iter().min().unwrap();
    let max = *pos.iter().max().unwrap();
    (min..=max)
        .map(|i| {
            pos.iter()
                .map(|p| {
                    let diff = (p - i).abs() as usize;
                    (1 + diff) * diff / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() {
    let contnets = fs::read_to_string("input/day7.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().next().unwrap().split(",").collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    #[test]
    fn part1_test() {
        assert_eq!(37, calc(&DATA.split(",").collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(168, calc2(&DATA.split(",").collect()));
    }
}
