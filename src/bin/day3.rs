use std::fs;

fn count_occurence(data: &Vec<&str>) -> Vec<(i32, i32)> {
    let mut count = Vec::new();
    count.resize(data[0].len(), (0, 0));
    for line in data {
        for (i, v) in line.chars().into_iter().enumerate() {
            match v {
                '0' => count[i].0 += 1,
                '1' => count[i].1 += 1,
                _ => panic!(""),
            }
        }
    }
    count
}
fn merge(nums: &Vec<u32>) -> u32 {
    nums.iter()
        .enumerate()
        .fold(0, |s, (i, v)| s | (v << nums.len() - 1 - i))
}
fn power_consumption(data: &Vec<&str>) -> (u32, u32) {
    let counts = count_occurence(data);
    let gamma: Vec<u32> = counts
        .iter()
        .map(|p| if p.0 > p.1 { 0 } else { 1 })
        .collect();
    let epsilon: Vec<u32> = counts
        .iter()
        .map(|p| if p.0 > p.1 { 1 } else { 0 })
        .collect();
    (merge(&gamma), merge(&epsilon))
}

fn life_support_rating(data: &Vec<&str>) -> (u32, u32) {
    let mut oxygen_data = data.clone();
    let mut current_digit = 0;
    while oxygen_data.len() > 1 {
        let (zero_set, one_set): (Vec<_>, Vec<_>) = oxygen_data
            .iter()
            .partition(|&line| line.chars().nth(current_digit).unwrap() == '0');
        oxygen_data = if zero_set.len() > one_set.len() {
            zero_set
        } else {
            one_set
        };
        current_digit += 1;
    }

    let mut co2_data = data.clone();
    let mut current_digit = 0;
    while co2_data.len() > 1 {
        let (zero_set, one_set): (Vec<_>, Vec<_>) = co2_data
            .iter()
            .partition(|&line| line.chars().nth(current_digit).unwrap() == '0');
        co2_data = if zero_set.len() <= one_set.len() {
            zero_set
        } else {
            one_set
        };
        current_digit += 1;
    }

    (
        u32::from_str_radix(oxygen_data[0], 2).unwrap(),
        u32::from_str_radix(co2_data[0], 2).unwrap(),
    )
}

fn main() {
    let contnets = fs::read_to_string("input/day3.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = power_consumption(&lines);
    println!("gamma: {} epsilon: {}", n.0, n.1);
    println!("power consumption: {}", n.0 * n.1);

    let n = life_support_rating(&lines);
    println!("oxygen: {} co2: {}", n.0, n.1);
    println!("life support rating: {}", n.0 * n.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn power_consumption_test() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!((22, 9), power_consumption(&data));
    }

    #[test]
    fn life_support_rating_test() {
        let data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!((23, 10), life_support_rating(&data));
    }
}
