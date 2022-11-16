use std::fs;

fn main() {
    let contnets = fs::read_to_string("aoc2020/input/day3.txt").unwrap();
    let map: Vec<Vec<char>> = contnets
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let tree_count = slop_down(&map, 3, 1);
    println!("part1 ans={}", tree_count);

    println!(
        "part2 ans={}",
        slop_down(&map, 1, 1)
            * slop_down(&map, 3, 1)
            * slop_down(&map, 5, 1)
            * slop_down(&map, 7, 1)
            * slop_down(&map, 1, 2)
    );
}

fn slop_down(map: &Vec<Vec<char>>, offset_x: usize, offset_y: usize) -> i32 {
    let width = map[0].len();
    let mut x = offset_x;
    let mut y = offset_y;
    let mut tree_count = 0;
    while y < map.len() {
        if map[y][x] == '#' {
            tree_count += 1;
        }
        x = (x + offset_x) % width;
        y += offset_y;
    }
    tree_count
}
