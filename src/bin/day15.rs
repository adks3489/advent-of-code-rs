use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
fn dijkstra(adj_list: &Vec<Vec<u32>>) -> usize {
    let max = adj_list.len();
    let goal = (max - 1, max - 1);
    let mut dist = vec![vec![usize::MAX; max]; max];
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: (0, 0),
    });
    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return cost;
        }
        if cost > dist[position.0][position.1] {
            continue;
        }

        for (x, y) in [
            (position.0 as i64 + 1, position.1 as i64),
            (position.0 as i64 - 1, position.1 as i64),
            (position.0 as i64, position.1 as i64 + 1),
            (position.0 as i64, position.1 as i64 - 1),
        ] {
            if x < 0 || x > max as i64 - 1 || y < 0 || y > max as i64 - 1 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;
            let next_cost = cost + adj_list[x][y] as usize;
            if next_cost < dist[x][y] {
                heap.push(State {
                    cost: next_cost,
                    position: (x, y),
                });
                dist[x][y] = next_cost;
            }
        }
    }
    0
}

fn calc(data: &Vec<&str>) -> usize {
    let distance: Vec<Vec<u32>> = data
        .iter()
        .map(|n| n.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    dijkstra(&distance)
}

fn increase(n: u32) -> u32 {
    match n {
        9 => 1,
        _ => n + 1,
    }
}
fn calc2(data: &Vec<&str>) -> usize {
    let mut distance: Vec<Vec<u32>> = data
        .iter()
        .map(|n| n.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let max = distance.len();
    distance.iter_mut().for_each(|row| row.resize(max * 5, 0));
    distance.resize(max * 5, vec![0; max * 5]);
    for y in 0..distance.len() {
        for x in 0..distance.len() {
            if x >= max {
                distance[y][x] = increase(distance[y][x - max]);
            } else if y >= max {
                if x < max {
                    distance[y][x] = increase(distance[y - max][x]);
                } else {
                    distance[y][x] = increase(distance[y][x]);
                }
            }
        }
    }
    dijkstra(&distance)
}

fn main() {
    let contnets = fs::read_to_string("input/day15.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn part1_test() {
        assert_eq!(40, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(315, calc2(&DATA.lines().collect()));
    }
}
