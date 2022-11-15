use std::fs;

fn calc(data: &Vec<&str>) -> u32 {
    let map: Vec<Vec<u32>> = data
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let y_max = map.len();
    let x_max = map[0].len();
    let mut sum = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if (y > 0 && *n >= map[y - 1][x])
                || (y < y_max - 1 && *n >= map[y + 1][x])
                || (x > 0 && *n >= map[y][x - 1])
                || (x < x_max - 1 && *n >= map[y][x + 1])
            {
                continue;
            }
            sum += n + 1;
        }
    }
    sum
}

fn dfs(map: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    if visited[y][x] {
        return 0;
    }
    visited[y][x] = true;

    let mut count = 1_usize;
    if y > 0 {
        count += dfs(map, visited, x, y - 1);
    }
    if y < map.len() - 1 {
        count += dfs(map, visited, x, y + 1);
    }
    if x > 0 {
        count += dfs(map, visited, x - 1, y);
    }
    if x < map[0].len() - 1 {
        count += dfs(map, visited, x + 1, y);
    }
    count
}

fn calc2(data: &Vec<&str>) -> usize {
    let map: Vec<Vec<u32>> = data
        .iter()
        .map(|r| {
            r.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let y_max = map.len();
    let x_max = map[0].len();
    let mut counts = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            if (y > 0 && *n >= map[y - 1][x])
                || (y < y_max - 1 && *n >= map[y + 1][x])
                || (x > 0 && *n >= map[y][x - 1])
                || (x < x_max - 1 && *n >= map[y][x + 1])
            {
                continue;
            }
            let mut visited: Vec<Vec<bool>> = map
                .iter()
                .map(|r| r.iter().map(|n| *n == 9).collect())
                .collect();
            let count = dfs(&map, &mut visited, x, y);
            counts.push(count);
        }
    }
    counts.sort();
    counts[counts.len() - 3..].iter().product()
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day9.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn part1_test() {
        assert_eq!(15, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(1134, calc2(&DATA.lines().collect()));
    }
}
