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

    let mut count = 0;
    for i in 3..lines.len() {
        if lines[i].parse::<i32>().unwrap() > lines[i - 3].parse::<i32>().unwrap() {
            count = count + 1;
        }
    }
    println!("{}", count);
}

enum Direction {
    Forward,
    Up,
    Down,
}
struct Command {
    direction: Direction,
    distance: i32,
}
impl Command {
    fn new(input: &str) -> Self {
        let col: Vec<&str> = input.split(" ").collect();
        Command {
            direction: match col[0] {
                "forward" => Direction::Forward,
                "up" => Direction::Up,
                "down" => Direction::Down,
                _ => panic!("unknown direction: {}", col[0]),
            },
            distance: col[1].parse::<i32>().unwrap(),
        }
    }
}
fn day2() {
    let contnets = fs::read_to_string("input/day2.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();

    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    lines
        .iter()
        .map(|l| Command::new(l))
        .for_each(|c| match c.direction {
            Direction::Forward => {
                horizontal_position += c.distance;
                depth += aim * c.distance;
            }
            Direction::Up => aim -= c.distance,
            Direction::Down => aim += c.distance,
        });
    println!("{}", horizontal_position * depth);
}

fn main() {
    day2();
}
