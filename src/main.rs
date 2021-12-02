use std::fs;

fn day1() {
    let contnets = fs::read_to_string("input/day1.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let mut count = 0;
    for i in 1..lines.len() {
        if lines[i].parse::<i32>().unwrap() > lines[i - 1].parse::<i32>().unwrap() {
            count = count + 1;
        }
    }
    println!("{}", count);
}

fn main() {
    day1();
}
