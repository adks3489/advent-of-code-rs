use std::fs;

fn calc(data: &Vec<&str>) -> usize {
    let mut pos: Vec<_> = data.iter().map(|n| n.parse::<i32>().unwrap()).collect();
    pos.sort();
    pos.iter()
        .map(|p| ((p - pos[pos.len() / 2]).abs() as usize))
        .sum()
}

fn calc2(data: &Vec<&str>) -> usize {
    let pos: Vec<i32> = data.iter().map(|n| n.parse::<i32>().unwrap()).collect();
    let mean = pos.iter().sum::<i32>() / pos.len() as i32;
    let spend = |n: i32| -> usize {
        pos.iter()
            .map(|p| {
                let diff = (p - n).abs() as usize;
                (1 + diff) * diff / 2
            })
            .sum()
    };
    spend(mean).min(spend(mean + 1))
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day7.txt").unwrap();
    let data: Vec<&str> = contnets.lines().next().unwrap().split(",").collect();
    let n = calc(&data);
    println!("Part1: {}", n);

    let n = calc2(&data);
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
