use std::fs;

fn count_larger(depths: &Vec<u32>) -> i32 {
    let mut count = 0;
    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count = count + 1;
        }
    }
    count
}

fn count_larger_window(depths: &Vec<u32>) -> i32 {
    let mut count = 0;
    for i in 3..depths.len() {
        if depths[i] > depths[i - 3] {
            count = count + 1;
        }
    }
    count
}

fn main() {
    let contnets = fs::read_to_string("input/day1.txt").unwrap();
    let depths: Vec<u32> = contnets.lines().map(|w| w.parse().unwrap()).collect();

    println!("larger than previous: {}", count_larger(&depths));
    println!(
        "larger than previous window: {}",
        count_larger_window(&depths)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_larger_test() {
        let test_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_larger(&test_data));
    }

    #[test]
    fn count_larger_window_test() {
        let test_data = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_larger_window(&test_data));
    }
}
