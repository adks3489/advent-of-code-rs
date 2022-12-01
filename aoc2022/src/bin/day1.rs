use std::fs;

fn main() {
    let contnets = fs::read_to_string("aoc2022/input/day1.txt").unwrap();
    let mut totals = Vec::new();
    let mut curr = 0;
    contnets.lines().for_each(|l| {
        if l.is_empty() {
            totals.push(curr);
            curr = 0;
        } else {
            curr += l.parse::<i32>().unwrap();
        }
    });
    totals.sort_by(|a, b| b.cmp(a));
    println!("Max: {}", totals[0]);
    println!(
        "Total of top 3({:?}): {}",
        &totals[0..3],
        &totals[0..3].iter().sum::<i32>()
    );
}
