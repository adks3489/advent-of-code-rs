use std::fs;

fn simulation(days: i32, ages: &Vec<&str>) -> usize {
    let mut ages_count: [usize; 9] = [0; 9];
    ages.iter()
        .map(|n| n.parse::<i32>().unwrap())
        .for_each(|n| {
            ages_count[n as usize] += 1;
        });
    for _day in 0..days {
        let mut next = [0; 9];
        for (n, count) in ages_count.into_iter().enumerate() {
            if n == 0 {
                next[6] = count;
                next[8] = count;
            } else {
                next[n - 1] += count;
            }
        }
        ages_count = next;
    }
    ages_count.iter().sum()
}

fn main() {
    let contnets = fs::read_to_string("input/day6.txt").unwrap();
    let line: Vec<&str> = contnets.lines().next().unwrap().split(",").collect();
    println!("Part1: {}", simulation(80, &line));

    println!("Part2: {}", simulation(256, &line));
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"3,4,3,1,2"#;

    #[test]
    fn part1_test() {
        assert_eq!(5934, simulation(80, &DATA.split(",").collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(26984457539, simulation(256, &DATA.split(",").collect()));
    }
}
