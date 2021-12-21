use std::fs;

fn expand(input: &mut Vec<Vec<u8>>) {
    let bound = input.len();
    input.iter_mut().into_iter().for_each(|row| {
        row.insert(0, 0);
        row.push(0);
    });
    input.insert(0, vec![0; bound + 2]);
    input.push(vec![0; bound + 2]);
}

fn extract_to_num(s: &[u8; 9]) -> usize {
    s.iter().fold(0_usize, |sum, n| sum << 1 | *n as usize)
}

fn get_pixels(input: &Vec<Vec<u8>>, bound: usize, x: usize, y: usize) -> [u8; 9] {
    let x = x as isize;
    let y = y as isize;
    let adjecents = [
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ];
    adjecents.map(|cood| {
        if cood.0 < 0 || cood.0 > bound as isize - 1 || cood.1 < 0 || cood.1 > bound as isize - 1 {
            0
        } else {
            input[cood.0 as usize][cood.1 as usize]
        }
    })
}

fn calc(data: &str, time: usize) -> usize {
    let (algorithm, input) = data.split_once("\n\n").unwrap();
    let algorithm: [u8; 512] = algorithm
        .chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!(),
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut input: Vec<Vec<u8>> = input
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    for _ in 0..time {
        expand(&mut input);
        let bound = input.len();
        let mut output = vec![vec![0; bound]; bound];
        for y in 0..bound {
            for x in 0..bound {
                let pos = extract_to_num(&get_pixels(&input, bound, x, y));
                output[y][x] = algorithm[pos];
                //println!("({},{}) {} {}", x, y, pos, output[y][x]);
            }
        }
        input = output;
    }
    input
        .iter()
        .fold(0_usize, |sum, row| sum + row.iter().sum::<u8>() as usize)
}

fn main() {
    let contnets = fs::read_to_string("input/day20.txt").unwrap();
    let n = calc(&contnets, 2);
    println!("Part1: {} 5225", n);

    let n = calc(&contnets, 50);
    println!("Part2: {} 18131", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_to_num_test() {
        assert_eq!(34, extract_to_num(&[0, 0, 0, 1, 0, 0, 0, 1, 0]));
    }

    static DATA: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

    #[test]
    fn part1_test() {
        assert_eq!(35, calc(&DATA, 2));
    }

    #[test]
    fn part2_test() {
        assert_eq!(3351, calc(&DATA, 50));
    }
}
