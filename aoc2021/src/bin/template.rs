use std::fs;

fn calc(data: &Vec<&str>) -> i32 {
    0
}

fn calc2(data: &Vec<&str>) -> i32 {
    0
}

fn main() {
    let contnets = fs::read_to_string("2021/inputinput.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#""#;

    #[test]
    fn part1_test() {
        assert_eq!(0, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(0, calc2(&DATA.lines().collect()));
    }
}
