use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    fn new(input: &str) -> Self {
        let col: Vec<&str> = input.split(" ").collect();
        match col[0] {
            "forward" => Command::Forward(col[1].parse().unwrap()),
            "up" => Command::Up(col[1].parse().unwrap()),
            "down" => Command::Down(col[1].parse().unwrap()),
            _ => panic!("unknown direction: {}", col[0]),
        }
    }
}

fn calc_position(commands: impl Iterator<Item = Command>) -> (i32, i32) {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    commands.for_each(|c| match c {
        Command::Forward(distance) => {
            horizontal_position += distance;
            depth += aim * distance;
        }
        Command::Up(distance) => aim -= distance,
        Command::Down(distance) => aim += distance,
    });
    (horizontal_position, depth)
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day2.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();

    let (horizontal_position, depth) = calc_position(lines.iter().map(|l| Command::new(l)));
    println!("{}", horizontal_position * depth);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn command_new() {
        assert_eq!(Command::Forward(1), Command::new("forward 1"));
        assert_eq!(Command::Down(2), Command::new("down 2"));
        assert_eq!(Command::Up(3), Command::new("up 3"));
    }

    #[test]
    fn calc_position_test() {
        assert_eq!(
            (15, 60),
            calc_position(
                vec![
                    "forward 5",
                    "down 5",
                    "forward 8",
                    "up 3",
                    "down 8",
                    "forward 2",
                ]
                .iter()
                .map(|l| Command::new(l)),
            )
        );
    }
}
