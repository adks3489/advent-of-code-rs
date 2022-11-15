use std::fs;

struct Slot {
    pub num: i32,
    pub marked: bool,
}
impl Slot {
    fn new(num: i32) -> Self {
        Slot { num, marked: false }
    }
}
#[derive(PartialEq, Eq)]
enum State {
    Finished,
    Unfinished,
}
struct Board(Vec<Vec<Slot>>, State);
impl Board {
    fn new(rows: &[&str]) -> Self {
        Board(
            rows.iter()
                .skip(1)
                .map(|row| {
                    row.split_whitespace()
                        .map(|c| Slot::new(c.parse().unwrap()))
                        .collect::<Vec<_>>()
                })
                .collect(),
            State::Unfinished,
        )
    }
    fn mark(&mut self, num: i32) -> bool {
        for row in &mut self.0 {
            match row.iter_mut().position(|s| s.num == num) {
                Some(pos) => {
                    row[pos].marked = true;
                    // check same row
                    if row.iter().all(|s| s.marked) {
                        return true;
                    }
                    // chekc same column
                    return self.0.iter().all(|row| row[pos].marked);
                }
                None => false,
            };
        }
        false
    }
    fn unmarked_sum(&self) -> i32 {
        self.0.iter().fold(0i32, |sum, row| {
            let row_sum: i32 = row.iter().filter(|s| !s.marked).map(|s| s.num).sum();
            sum + row_sum
        })
    }
}

fn parse(data: &Vec<&str>) -> (Vec<i32>, Vec<Board>) {
    let draw_nums: Vec<i32> = data[0].split(",").map(|n| n.parse().unwrap()).collect();
    let boards: Vec<Board> = data[1..]
        .windows(6)
        .step_by(6)
        .map(|rows| Board::new(rows))
        .collect();
    (draw_nums, boards)
}

fn guess_first_win(data: &Vec<&str>) -> i32 {
    let (draw_nums, mut boards) = parse(&data);
    for draw in draw_nums {
        for board in &mut boards {
            if board.mark(draw) {
                return draw * board.unmarked_sum();
            }
        }
    }
    0
}

fn guess_last_win(data: &Vec<&str>) -> i32 {
    let (draw_nums, mut boards) = parse(&data);
    for draw in draw_nums {
        for board in &mut boards {
            board.1 = match board.mark(draw) {
                true => State::Finished,
                false => State::Unfinished,
            };
        }
        if boards.len() == 1 && boards[0].1 == State::Finished {
            return draw * boards[0].unmarked_sum();
        }
        boards = boards
            .into_iter()
            .filter(|b| b.1 == State::Unfinished)
            .collect();
    }
    0
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day4.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();

    let score = guess_first_win(&lines);
    println!("First win score: {}", score);

    let score = guess_last_win(&lines);
    println!("Last winscore: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
  8  2 23  4 24
21  9 14 16  7
  6 10  3 18  5
  1 12 20 15 19

  3 15  0  2 22
  9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
  2  0 12  3  7"#;

    #[test]
    fn first_win_test() {
        assert_eq!(4512, guess_first_win(&DATA.lines().collect()));
    }

    #[test]
    fn last_win_test() {
        assert_eq!(1924, guess_last_win(&DATA.lines().collect()));
    }
}
