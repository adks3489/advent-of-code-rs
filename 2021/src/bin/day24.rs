use std::fs;

type VarIdx = usize;
fn parse_var_idx(word: &str) -> VarIdx {
    match word {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!(),
    }
}
#[derive(Debug)]
enum Source {
    Num(i64),
    Var(usize),
}
impl Source {
    fn new(word: &str) -> Self {
        match word {
            "w" | "x" | "y" | "z" => Source::Var(parse_var_idx(word)),
            _ => Source::Num(word.parse().unwrap()),
        }
    }
}
#[derive(Debug)]
enum Instruction {
    INP(VarIdx),
    ADD(VarIdx, Source),
    MUL(VarIdx, Source),
    DIV(VarIdx, Source),
    MOD(VarIdx, Source),
    EQL(VarIdx, Source),
}
impl Instruction {
    fn new(line: &str) -> Self {
        let mut part = line.split(" ");
        match part.next().unwrap() {
            "inp" => Instruction::INP(parse_var_idx(part.next().unwrap())),
            "add" => Instruction::ADD(
                parse_var_idx(part.next().unwrap()),
                Source::new(part.next().unwrap()),
            ),
            "mul" => Instruction::MUL(
                parse_var_idx(part.next().unwrap()),
                Source::new(part.next().unwrap()),
            ),
            "div" => Instruction::DIV(
                parse_var_idx(part.next().unwrap()),
                Source::new(part.next().unwrap()),
            ),
            "mod" => Instruction::MOD(
                parse_var_idx(part.next().unwrap()),
                Source::new(part.next().unwrap()),
            ),
            "eql" => Instruction::EQL(
                parse_var_idx(part.next().unwrap()),
                Source::new(part.next().unwrap()),
            ),
            _ => panic!(),
        }
    }
}
struct ALU {
    input: String,
    input_indicator: usize,
    vars: [i64; 4],
}
impl ALU {
    fn new(input: &str) -> Self {
        ALU {
            input: input.to_string(),
            input_indicator: 0,
            vars: [0; 4],
        }
    }
    fn execute(&mut self, instruction: &Instruction) -> bool {
        match instruction {
            Instruction::INP(var) => {
                self.set(
                    var,
                    self.input
                        .chars()
                        .nth(self.input_indicator)
                        .unwrap()
                        .to_digit(10)
                        .unwrap() as i64,
                );
                self.input_indicator += 1;
            }
            Instruction::ADD(target, source) => {
                self.set(target, self.get(target) + self.extract_val(source))
            }
            Instruction::MUL(target, source) => {
                self.set(target, self.get(target) * self.extract_val(source))
            }
            Instruction::DIV(target, source) => {
                let b = self.extract_val(source);
                if b == 0 {
                    return false;
                }
                self.set(target, self.get(target) / b)
            }
            Instruction::MOD(target, source) => {
                let a = self.get(target);
                let b = self.extract_val(source);
                if a < 0 || b <= 0 {
                    return false;
                }
                self.set(target, a % b)
            }
            Instruction::EQL(target, source) => self.set(
                target,
                if self.get(target) == self.extract_val(source) {
                    1
                } else {
                    0
                },
            ),
        }
        true
    }
    fn extract_val(&self, target: &Source) -> i64 {
        match target {
            Source::Num(n) => *n,
            Source::Var(var) => self.get(var),
        }
    }
    fn get(&self, source: &VarIdx) -> i64 {
        self.vars[*source]
    }
    fn set(&mut self, target: &VarIdx, val: i64) {
        self.vars[*target] = val;
    }
}

fn is_valid_model(model_num: &str, instructions: &Vec<Instruction>) -> bool {
    if model_num.to_string().contains("0") {
        return false;
    }
    let mut alu = ALU::new(model_num);
    for instruction in instructions.iter() {
        if !alu.execute(instruction) {
            return false;
        }
    }
    alu.vars[3] == 0
}

fn parse(data: &Vec<&str>) -> Vec<Instruction> {
    let instructions: Vec<_> = data.iter().map(|line| Instruction::new(line)).collect();
    instructions
}

fn calc_raw(data: &Vec<&str>) -> u64 {
    let instructions = parse(data);
    let mut model_num = 9999999_9999999;
    loop {
        if model_num % 100000 == 0 {
            println!("try {}", model_num);
        }
        if is_valid_model(&model_num.to_string(), &instructions) {
            break;
        }
        model_num -= 1;
    }
    model_num
}

// (max_valid)
fn calc_digit(i: usize, prev_z: i64) -> Option<u64> {
    let div_z = [1, 1, 1, 26, 1, 1, 26, 26, 26, 1, 1, 26, 26, 26];
    let add_x = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
    let add_y = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];

    for w in [9, 8, 7, 6, 5, 4, 3, 2, 1] {
        // 26 radix
        let x = prev_z % 26 + add_x[i];
        let z = if x != w as i64 {
            (prev_z / div_z[i]) * 26 + w as i64 + add_y[i]
        } else {
            prev_z / div_z[i]
        };
        if i == 13 {
            if z == 0 {
                print!("{}", w);
                return Some(w as u64);
            }
            return None;
        }
        if let Some(next_digit) = calc_digit(i + 1, z) {
            println!("{} {}", w, next_digit);
            return Some(w as u64 * 10_u64.pow(13 - i as u32) + next_digit);
        }
    }
    None
}
fn calc() -> u64 {
    calc_digit(0, 0).unwrap()
}
fn calc0(data: &Vec<&str>) -> u64 {
    //let instructions = parse(data);
    let div_z = [1, 1, 1, 26, 1, 1, 26, 26, 26, 1, 1, 26, 26, 26];
    let add_x = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
    let add_y = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];
    let mut model_num = 9999999_9999999;
    loop {
        if model_num % 1000000 == 0 {
            println!("try {}", model_num);
        }
        let s = model_num.to_string();
        if !s.contains("0") {
            let mut z = 0;
            for (i, digit) in s.chars().enumerate() {
                let w = digit.to_digit(10).unwrap() as i64;
                let x = z % 26 + add_x[i];
                if x != w {
                    z = (z / div_z[i]) * 26 + w + add_y[i];
                } else {
                    z = z / div_z[i];
                }
            }
            if z == 0 {
                break;
            }
        }
        model_num -= 1;
    }
    model_num
}

fn calc1(data: &Vec<&str>) -> u64 {
    let instructions = parse(data);
    let div_z = [1, 1, 1, 26, 1, 1, 26, 26, 26, 1, 1, 26, 26, 26];
    let add_x = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
    let add_y = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];
    /*
       x = (prev_z % 26 + A) != w
       z = (prev_z / N) * (25x + 1) + (W+B)x
    */
    let mut digits = Vec::new();
    for digit_instructions in instructions.chunks(18) {
        for i in (1..=9).rev() {
            let mut alu = ALU::new(&i.to_string());
            for instruction in digit_instructions.iter() {
                alu.execute(instruction);
            }
            println!("{} z:{}", i, alu.vars[3]);
            if alu.vars[3] == 0 {
                digits.push(i as u64);
                break;
            }
        }
    }
    digits.into_iter().reduce(|sum, n| sum << 1 | n).unwrap()
}

fn calc_re(instructions: &Vec<Instruction>, i: usize, mut model: [u8; 14]) -> Option<u64> {
    let div_z = [1, 1, 1, 26, 1, 1, 26, 26, 26, 1, 1, 26, 26, 26];
    let add_x = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
    let add_y = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];
    // w4 = w3+2
    // w7 = w6+1
    // w8 = w5+5
    // w9 = w2-1
    // w12 = w11+8
    // w13 = w10-2
    // w14 = w1-8
    let candicates = [
        Some(9..=9),
        Some(2..=9),
        Some(1..=3),
        None,
        Some(1..=4),
        Some(1..=8),
        None,
        None,
        None,
        Some(3..=9),
        Some(1..=1),
        None,
        None,
        None,
    ];

    match candicates[i].clone() {
        Some(range) => {
            for w in range {
                model[i] = w;
                if let Some(n) = calc_re(instructions, i + 1, model.clone()) {
                    return Some(n);
                }
            }
            None
        }
        None => {
            match i {
                13 => {
                    model[i] = model[0] - 8;
                }
                3 => {
                    model[i] = model[2] + 6;
                }
                6 => {
                    model[i] = model[5] + 1;
                }
                7 => {
                    model[i] = model[4] + 5;
                }
                8 => {
                    model[i] = model[1] - 1;
                }
                11 => {
                    model[i] = model[10] + 8;
                }
                12 => {
                    model[i] = model[9] - 2;
                }
                _ => panic!(),
            }
            if i == 13 {
                println!(
                    "{}",
                    model.iter().fold(0_u64, |sum, n| sum * 10 + *n as u64)
                );
                if is_valid_model(
                    &model.iter().fold(String::new(), |mut s, n| {
                        s += &n.to_string();
                        s
                    }),
                    instructions,
                ) {
                    Some(model.iter().fold(0_u64, |sum, n| sum * 10 + *n as u64))
                } else {
                    None
                }
            } else {
                calc_re(instructions, i + 1, model)
            }
        }
    }
}

fn calc2(data: &Vec<&str>) -> i32 {
    0
}

fn main() {
    let contnets = fs::read_to_string("2021/input/day24.txt").unwrap();
    let lines: Vec<&str> = contnets.lines().collect();
    let ins = parse(&contnets.lines().collect());
    let n = calc_re(&ins, 0, [0; 14]).unwrap();
    println!("Part1: {}", n);

    let n = calc2(&lines);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_valid_model_test() {
        let contnets = fs::read_to_string("2021/input/day24.txt").unwrap();
        let ins = parse(&contnets.lines().collect());
        //assert_eq!(true, is_valid_model("99994899491979", &ins));
        assert_eq!(true, is_valid_model("99394899891971", &ins));
    }
    #[test]
    fn test1() {
        const DATA: &str = r#"inp z
inp x
mul z 3
eql z x"#;
        let ins = parse(&DATA.lines().collect());
        let mut alu = ALU::new("39");
        for instruction in ins.iter() {
            if !alu.execute(instruction) {
                break;
            }
        }
        assert_eq!(1, alu.vars[3]);

        let mut alu = ALU::new("49");
        for instruction in ins.iter() {
            if !alu.execute(instruction) {
                break;
            }
        }
        assert_eq!(0, alu.vars[3]);
    }
    #[test]
    fn part2_test() {
        //assert_eq!(0, calc2());
    }
}
