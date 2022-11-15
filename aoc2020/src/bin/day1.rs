use std::collections::HashSet;
use std::fs;

fn main() {
    let contnets = fs::read_to_string("aoc2020/input/day1.txt").unwrap();
    let nums = contnets.lines().map(|w| w.parse::<u32>().unwrap());
    let (v1, v2) = find_two_sum_to_2020(nums.clone());
    println!("part1 ans: {} * {} = {}", v1, v2, v1 * v2);
    let (v1, v2, v3) = find_three_sum_to_2020(nums);
    println!("part2 ans: {} * {} * {} = {}", v1, v2, v3, v1 * v2 * v3);
}

fn find_two_sum_to_2020(nums: impl Iterator<Item = u32>) -> (u32, u32) {
    let mut prev_nums = HashSet::new();
    for n in nums {
        let target = 2020 - n;
        if prev_nums.contains(&target) {
            return (target, n);
        }
        prev_nums.insert(n);
    }
    panic!("not found");
}

fn find_three_sum_to_2020(nums: impl Iterator<Item = u32>) -> (u32, u32, u32) {
    let nums: Vec<u32> = nums.collect();
    let prev_nums: HashSet<u32> = nums.clone().into_iter().collect();
    let len = nums.len();
    for i in 0..len {
        for j in i..len {
            if (nums[i] + nums[j]) > 2020 {
                continue;
            }
            let target = 2020 - (nums[i] + nums[j]);
            if prev_nums.contains(&target) {
                return (target, nums[i], nums[j]);
            }
        }
    }
    panic!("not found");
}
