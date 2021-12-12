use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn is_small_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}

fn bfs<'a>(
    connections: &'a HashMap<String, HashSet<String>>,
    node: &'a str,
    from_path: &Vec<&'a str>,
) -> Vec<Vec<&'a str>> {
    if node == "end" {
        return vec![vec![node]];
    }
    if is_small_cave(node) && from_path.contains(&node) {
        return vec![];
    }

    let mut from_path = from_path.clone();
    from_path.push(node);
    connections[node]
        .iter()
        .flat_map(|next| {
            let mut sub_path = bfs(connections, &next, &from_path);
            sub_path.iter_mut().for_each(|path| {
                path.insert(0, node);
            });
            sub_path
        })
        .collect()
}

fn calc(data: &Vec<&str>) -> usize {
    let connections = parse(data);
    let pathes = bfs(&connections, "start", &Vec::new());
    pathes.len()
}

fn bfs2<'a>(
    connections: &'a HashMap<String, HashSet<String>>,
    node: &'a str,
    from_path: &Vec<&'a str>,
    has_twiced: bool,
) -> Vec<Vec<&'a str>> {
    if node == "end" {
        return vec![vec![node]];
    }
    let mut twiced = has_twiced;
    if is_small_cave(node) && from_path.contains(&node) {
        if has_twiced {
            return vec![];
        }
        twiced = true;
    };

    let mut from_path = from_path.clone();
    from_path.push(node);
    connections[node]
        .iter()
        .flat_map(|next| {
            let mut sub_path = bfs2(connections, &next, &from_path, twiced);
            sub_path.iter_mut().for_each(|path| {
                path.insert(0, node);
            });
            sub_path
        })
        .collect()
}
fn calc2(data: &Vec<&str>) -> usize {
    let connections = parse(data);
    let pathes = bfs2(&connections, "start", &Vec::new(), false);
    let small_caves: Vec<&str> = connections
        .iter()
        .filter(|(k, _)| is_small_cave(k))
        .map(|(k, _)| k.as_str())
        .collect();
    pathes
        .iter()
        .filter(|path| {
            small_caves
                .iter()
                .filter(|small_cave| path.iter().filter(|cave| cave == small_cave).count() >= 2)
                .count()
                <= 1
        })
        .count()
}

fn parse(data: &Vec<&str>) -> HashMap<String, HashSet<String>> {
    let mut connections = HashMap::new();
    data.iter().for_each(|l| {
        let (a, b) = l.split_once("-").unwrap();
        if b != "start" {
            connections
                .entry(a.to_string())
                .or_insert(HashSet::new())
                .insert(b.to_string());
        }
        if a != "start" {
            connections
                .entry(b.to_string())
                .or_insert(HashSet::new())
                .insert(a.to_string());
        }
    });
    connections
}

fn main() {
    let contnets = fs::read_to_string("input/day12.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

    #[test]
    fn part1_test() {
        assert_eq!(10, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(36, calc2(&DATA.lines().collect()));
    }
}
