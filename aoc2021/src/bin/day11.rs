use std::fs;

fn increase_by_adjacent(n: &mut u32) {
    if *n != 0 {
        *n += 1
    }
}

fn flashes(energy: &mut Vec<Vec<u32>>) -> usize {
    let mut total = 0;
    loop {
        let mut count = 0;
        for y in 0..10 {
            for x in 0..10 {
                if energy[y][x] > 9 {
                    count += 1;
                    energy[y][x] = 0;
                    if y > 0 {
                        if x > 0 {
                            increase_by_adjacent(&mut energy[y - 1][x - 1]);
                        }
                        increase_by_adjacent(&mut energy[y - 1][x]);
                        if x < 9 {
                            increase_by_adjacent(&mut energy[y - 1][x + 1]);
                        }
                    }
                    if x > 0 {
                        increase_by_adjacent(&mut energy[y][x - 1]);
                    }
                    if x < 9 {
                        increase_by_adjacent(&mut energy[y][x + 1]);
                    }
                    if y < 9 {
                        if x > 0 {
                            increase_by_adjacent(&mut energy[y + 1][x - 1]);
                        }
                        increase_by_adjacent(&mut energy[y + 1][x]);
                        if x < 9 {
                            increase_by_adjacent(&mut energy[y + 1][x + 1]);
                        }
                    }
                }
            }
        }
        if count == 0 {
            break;
        }
        total += count;
    }
    total
}

fn calc(data: &Vec<&str>) -> usize {
    let mut energy: Vec<Vec<u32>> = data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum = 0_usize;
    for _ in 0..100 {
        energy = energy
            .into_iter()
            .map(|row| row.iter().map(|n| n + 1).collect())
            .collect();
        sum += flashes(&mut energy);
    }
    sum
}

fn calc2(data: &Vec<&str>) -> i32 {
    let mut energy: Vec<Vec<u32>> = data
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut step = 0;
    loop {
        energy = energy
            .into_iter()
            .map(|row| row.iter().map(|n| n + 1).collect())
            .collect();
        if flashes(&mut energy) == 100 {
            return step + 1;
        }
        step += 1;
    }
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day11.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn part1_test() {
        assert_eq!(1656, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(195, calc2(&DATA.lines().collect()));
    }
}
