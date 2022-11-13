struct DeterministicDice {
    num: u32,
    roll_times: u32,
}
impl DeterministicDice {
    fn new() -> Self {
        DeterministicDice {
            num: 1,
            roll_times: 0,
        }
    }
    fn roll3(&mut self) -> u32 {
        let mut sum = 0;
        for _ in 0..3 {
            sum += self.num;
            if self.num == 100 {
                self.num = 1;
            } else {
                self.num += 1;
            }
            self.roll_times += 1;
        }
        sum
    }
}
#[derive(Copy, Clone, Debug)]
struct Player {
    position: u32,
    score: u32,
}
impl Player {
    fn new(position: u32) -> Self {
        Player { position, score: 0 }
    }
    fn move_pawn(&mut self, num: u32) {
        self.position += num;
        if self.position > 10 {
            let remain = self.position % 10;
            self.position = if remain == 0 { 10 } else { remain }
        }
        self.score += self.position
    }
}
fn calc(position: &[u32]) -> u32 {
    let mut dice = DeterministicDice::new();
    let mut player = [Player::new(position[0]), Player::new(position[1])];
    let win_score = 1000;
    loop {
        player[0].move_pawn(dice.roll3());
        if player[0].score >= win_score {
            break;
        }
        player[1].move_pawn(dice.roll3());
        if player[1].score >= win_score {
            break;
        }
    }
    player[0].score.min(player[1].score) * dice.roll_times
}

#[derive(Clone, Copy)]
struct Universe {
    player: [Player; 2],
    mirror: usize,
}
fn calc2(position: &[u32]) -> usize {
    let mut universe = vec![Universe {
        player: [Player::new(position[0]), Player::new(position[1])],
        mirror: 1,
    }];
    let win_score = 21;
    let mut win_count = [0_usize; 2];
    let mut player_turn = 0;
    loop {
        let origin_universe = universe;
        universe = Vec::with_capacity(origin_universe.len() * 7);
        for (num, mirror) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let mut universe_copy = origin_universe.clone();
            universe_copy.iter_mut().for_each(|u| {
                u.player[player_turn].move_pawn(num);
                u.mirror *= mirror;
            });
            universe.append(&mut universe_copy);
        }

        universe = universe
            .into_iter()
            .filter(|u| {
                if u.player[player_turn].score < win_score {
                    true
                } else {
                    win_count[player_turn] += u.mirror;
                    false
                }
            })
            .collect();
        player_turn = if player_turn == 0 { 1 } else { 0 };
        if universe.is_empty() {
            break;
        }
    }
    win_count.into_iter().max().unwrap()
}

fn main() {
    let n = calc(&[3, 4]);
    println!("Part1: {}", n);

    let n = calc2(&[3, 4]);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(739785, calc(&[4, 8]));
    }

    #[test]
    fn part2_test() {
        assert_eq!(444356092776315, calc2(&[4, 8]));
    }
}
