use std::fs;

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

fn main() {
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
