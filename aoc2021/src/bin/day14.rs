use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Rule {
    a: char,
    b: char,
    insert: char,
}
impl FromStr for Rule {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        Ok(Self {
            a: chars.next().unwrap(),
            b: chars.next().unwrap(),
            insert: chars.last().unwrap(),
        })
    }
}

fn calc(data: &str, step: usize) -> usize {
    let (template, rules) = parse(data);
    let mut rule_counts: HashMap<&Rule, usize> = HashMap::new();
    template.windows(2).for_each(|pair| {
        let rule = get_rule(&rules, pair[0], pair[1]);
        *rule_counts.entry(rule).or_insert(0) += 1;
    });
    for _ in 0..step {
        for (rule, count) in std::mem::take(&mut rule_counts).into_iter() {
            let rule1 = get_rule(&rules, rule.a, rule.insert);
            let rule2 = get_rule(&rules, rule.insert, rule.b);
            *rule_counts.entry(rule1).or_insert(0) += count;
            *rule_counts.entry(rule2).or_insert(0) += count;
        }
    }
    let mut counts = rule_counts
        .iter()
        .fold(HashMap::new(), |mut counts, (k, v)| {
            *counts.entry(k.a).or_insert(0_usize) += v;
            *counts.entry(k.b).or_insert(0) += v;
            counts
        });
    *counts.entry(*template.first().unwrap()).or_insert(0) += 1;
    *counts.entry(*template.last().unwrap()).or_insert(0) += 1;
    let it: Vec<_> = counts.iter().map(|(_, v)| *v).collect();
    (it.iter().max().unwrap() - it.iter().min().unwrap()) / 2
}

fn get_rule(rules: &Vec<Rule>, a: char, b: char) -> &Rule {
    let rule = rules
        .iter()
        .find(|rule| rule.a == a && rule.b == b)
        .unwrap();
    rule
}

fn parse(data: &str) -> (Vec<char>, Vec<Rule>) {
    let (template, rules) = data.split_once("\n\n").unwrap();
    let template: Vec<char> = template.chars().collect();
    let rules: Vec<Rule> = rules
        .lines()
        .map(|s| s.parse().expect("parse fail"))
        .collect();
    (template, rules)
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day14.txt").unwrap();
    let n = calc(&contnets, 10);
    println!("Part1: {}", n);

    let n = calc(&contnets, 40);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn part1_test() {
        assert_eq!(1588, calc(&DATA, 10));
    }

    #[test]
    fn part2_test() {
        assert_eq!(2188189693529, calc(&DATA, 40));
    }
}
