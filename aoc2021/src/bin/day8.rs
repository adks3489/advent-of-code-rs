use std::fs;

struct Entry {
    patterns: [String; 10],
    output: Vec<String>,
}
impl Entry {
    fn new(s: &str) -> Self {
        let mut p = s.split("|");
        let patterns = p.next().unwrap().split_whitespace();
        let mut digits = [""; 10];
        digits[1] = patterns.clone().find(|p| p.len() == 2).unwrap();
        digits[4] = patterns.clone().find(|p| p.len() == 4).unwrap();
        digits[7] = patterns.clone().find(|p| p.len() == 3).unwrap();
        digits[8] = patterns.clone().find(|p| p.len() == 7).unwrap();
        digits[3] = patterns
            .clone()
            .find(|p| p.len() == 5 && digits[1].chars().all(|c| p.contains(c)))
            .unwrap();
        digits[9] = patterns
            .clone()
            .find(|p| p.len() == 6 && digits[4].chars().all(|c| p.contains(c)))
            .unwrap();
        digits[0] = patterns
            .clone()
            .find(|p| p.len() == 6 && *p != digits[9] && digits[1].chars().all(|c| p.contains(c)))
            .unwrap();
        digits[6] = patterns
            .clone()
            .clone()
            .filter(|p| p.len() == 6 && *p != digits[0] && *p != digits[9])
            .next()
            .unwrap();
        let e = digits[0].chars().find(|p| !digits[9].contains(*p)).unwrap();
        digits[2] = patterns
            .clone()
            .find(|p| p.len() == 5 && *p != digits[3] && p.contains(e))
            .unwrap();
        digits[5] = patterns
            .clone()
            .find(|p| p.len() == 5 && *p != digits[2] && *p != digits[3])
            .unwrap();

        Self {
            patterns: digits.map(|s| s.to_string()),
            output: p
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        }
    }
    fn match_pattern(&self, s: &str) -> i32 {
        self.patterns
            .iter()
            .position(|d| d.len() == s.len() && d.chars().all(|c| s.contains(c)))
            .unwrap() as i32
    }
}

fn calc(data: &Vec<&str>) -> usize {
    let entries: Vec<_> = data.iter().map(|l| Entry::new(l)).collect();
    entries
        .iter()
        .map(|p| {
            p.output
                .iter()
                .map(|segs| segs.len())
                .filter(|d| *d == 2 || *d == 3 || *d == 4 || *d == 7)
                .count()
        })
        .sum()
}

fn calc2(data: &Vec<&str>) -> i32 {
    let entries: Vec<_> = data.iter().map(|l| Entry::new(l)).collect();
    entries
        .iter()
        .map(|e| {
            e.output
                .iter()
                .enumerate()
                .map(|(p, o)| e.match_pattern(&o) * 10_i32.pow((3 - p) as u32))
                .sum::<i32>()
        })
        .sum()
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day8.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let n = calc(&lines);
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    static DATA: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn part1_test() {
        assert_eq!(26, calc(&DATA.lines().collect()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(61229, calc2(&DATA.lines().collect()));
    }
}
