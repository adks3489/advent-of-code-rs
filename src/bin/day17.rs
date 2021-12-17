use std::{collections::HashSet, ops::RangeInclusive};

fn calc(_: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> i32 {
    let min = target_y.min().unwrap();
    // t=max(abs(target_y))
    // v0=t-1
    min * (min + 1) / 2
}

fn calc2(target_x: RangeInclusive<i32>, target_y: RangeInclusive<i32>) -> usize {
    let mut max_y = 0;
    let mut valid = HashSet::new();
    for init_yv in *target_y.start()..=-*target_y.start() {
        for init_xv in 0..=*target_x.end() {
            let mut x = 0;
            let mut y = 0;
            let mut xv = init_xv;
            let mut yv = init_yv;
            loop {
                x += xv;
                y += yv;
                max_y = max_y.max(y);
                if target_x.contains(&x) && target_y.contains(&y) {
                    valid.insert((init_xv, init_yv));
                    break;
                }
                if xv > 0 {
                    xv -= 1;
                } else if xv < 0 {
                    xv += 1;
                }
                yv -= 1;
                if y < *target_y.start() || (!target_x.contains(&x) && xv == 0) {
                    break;
                }
            }
        }
    }
    valid.len()
}

fn main() {
    let x = 60..=94;
    let y = -171..=-136;
    let n = calc(x.clone(), y.clone());
    println!("Part1: {}", n);

    let n = calc2(x, y);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(45, calc(20..=30, -10..=-5));
    }

    #[test]
    fn part2_test() {
        assert_eq!(112, calc2(20..=30, -10..=-5));
    }
}
