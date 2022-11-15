use std::fs;
pub fn part1(input: &str) -> usize {
    let (iea, image) = parse(input);
    let final_image = enhance_passes(iea, image, 1);
    count_lit(&final_image)
}

pub fn part2(input: &str) -> usize {
    let (iea, image) = parse(input);
    let final_image = enhance_passes(iea, image, 50);
    count_lit(&final_image)
}
// reimplemented using Vecs after completing it with HashMap cause they were really slow, about 7 seconds for part2
// original, slow, version is below
fn parse(input: &str) -> ([u8; 512], Vec<Vec<u8>>) {
    let mut iea = [0; 512];
    let mut image = vec![];
    let (algorithm, input) = input.split_once("\n\n").unwrap();
    for (i, c) in algorithm.chars().enumerate() {
        match c {
            '.' => iea[i] = 0,
            '#' => iea[i] = 1,
            _ => panic!("Unexpected character found"),
        }
    }
    for line in input.lines() {
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '.' => row.push(0),
                '#' => row.push(1),
                _ => panic!("Unexpected character found"),
            }
        }
        image.push(row);
    }
    (iea, image)
}

fn count_lit(image: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for r in image {
        for v in r {
            count += *v as usize;
        }
    }
    count
}

fn enhance_passes(iea: [u8; 512], mut image: Vec<Vec<u8>>, passes: u8) -> Vec<Vec<u8>> {
    for i in 0..passes {
        if i % 2 == 1 {
            image = enhance(iea, image, iea[0]);
        } else {
            image = enhance(iea, image, 0);
        }
    }
    image
}

fn enhance(iea: [u8; 512], image: Vec<Vec<u8>>, default: u8) -> Vec<Vec<u8>> {
    let mut new_image = vec![vec![0; image[0].len() + 2]; image.len() + 2];
    for r in 0..new_image.len() {
        for c in 0..new_image[0].len() {
            // edited a version found on reddit to not fail compilation with
            // subtraction with overflow, it's about 10ms slower
            let adjacents = adjacent(r as isize - 1, c as isize - 1);
            //println!("{:?}",adjacents);

            let i = adjacents.iter().fold(0, |n, &(r1, c1)| {
                if r1 < 0
                    || c1 < 0
                    || r1 >= (image.len() as isize)
                    || c1 >= (image[0].len() as isize)
                {
                    let x = default as usize;
                    n << 1 | x
                } else {
                    let x = image[(r1) as usize][(c1) as usize] as usize;
                    n << 1 | x
                }
            });
            new_image[r][c] = iea[i];
            println!("({},{}) {} {}", c, r, i, new_image[r][c]);
        }
    }
    new_image
}

fn adjacent(x: isize, y: isize) -> [(isize, isize); 9] {
    [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
    ]
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day20.txt").unwrap();
    let n = part1(&contnets);
    println!("Part1: {}", n);
    // let n = part2(&contnets);
    // println!("Part2: {}", n);
}
