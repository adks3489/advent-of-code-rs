use std::fs;
fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn to_num(bits: &[char]) -> usize {
    bits.iter().fold(0, |accum, c| match c {
        '1' => accum << 1 | 1,
        '0' => accum << 1,
        _ => panic!(),
    })
}

enum Packet {
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Min(Vec<Packet>),
    Max(Vec<Packet>),
    Literal(usize),
    Greater(Vec<Packet>),
    Less(Vec<Packet>),
    Equal(Vec<Packet>),
}
impl Packet {
    fn new_operator(t: usize, sub_packets: Vec<Packet>) -> Self {
        match t {
            0 => Packet::Sum(sub_packets),
            1 => Packet::Product(sub_packets),
            2 => Packet::Min(sub_packets),
            3 => Packet::Max(sub_packets),
            5 => Packet::Greater(sub_packets),
            6 => Packet::Less(sub_packets),
            7 => Packet::Equal(sub_packets),
            _ => panic!(),
        }
    }
    fn eval(&self) -> usize {
        match self {
            Packet::Sum(sub_packet) => sub_packet.iter().map(|p| p.eval()).sum(),
            Packet::Product(sub_packet) => sub_packet.iter().map(|p| p.eval()).product(),
            Packet::Min(sub_packet) => sub_packet.iter().map(|p| p.eval()).min().unwrap(),
            Packet::Max(sub_packet) => sub_packet.iter().map(|p| p.eval()).max().unwrap(),
            Packet::Literal(n) => *n,
            Packet::Greater(sub_packet) => {
                if sub_packet[0].eval() > sub_packet[1].eval() {
                    1
                } else {
                    0
                }
            }
            Packet::Less(sub_packet) => {
                if sub_packet[0].eval() < sub_packet[1].eval() {
                    1
                } else {
                    0
                }
            }
            Packet::Equal(sub_packet) => {
                if sub_packet[0].eval() == sub_packet[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_packet(bits: &Vec<char>, mut pos: usize) -> (usize, usize, Packet) {
    let ver = to_num(&bits[pos..pos + 3]);
    let mut ver_sum = ver;
    let t = to_num(&bits[pos + 3..pos + 6]);
    pos += 6;
    if t == 4 {
        let mut num = 0;
        loop {
            num = num << 4 | to_num(&bits[pos + 1..pos + 5]);
            pos += 5;
            if bits[pos - 5] == '0' {
                break;
            }
        }
        (pos, ver, Packet::Literal(num))
    } else {
        match bits[pos] {
            '0' => {
                pos += 1;
                let sub_packet_len: usize = to_num(&bits[pos..pos + 15]);
                pos += 15;
                let mut remain_sub_packet_len = sub_packet_len;
                let mut sub_packets = Vec::new();
                loop {
                    let (end_pos, ver, packet) = parse_packet(bits, pos);
                    ver_sum += ver;
                    remain_sub_packet_len -= end_pos - pos;
                    pos = end_pos;
                    sub_packets.push(packet);
                    if remain_sub_packet_len <= 0 {
                        break;
                    }
                }
                (pos, ver_sum, Packet::new_operator(t, sub_packets))
            }
            '1' => {
                pos += 1;
                let sub_packet_count = to_num(&bits[pos..pos + 11]);
                pos += 11;
                let mut sub_packets = Vec::new();
                for _ in 0..sub_packet_count {
                    let (end_pos, ver, packet) = parse_packet(bits, pos);
                    ver_sum += ver;
                    pos = end_pos;
                    sub_packets.push(packet);
                }
                (pos, ver_sum, Packet::new_operator(t, sub_packets))
            }
            _ => panic!(),
        }
    }
}

fn calc(data: &str) -> usize {
    let bits: Vec<char> = data.chars().flat_map(|c| to_binary(c).chars()).collect();
    let (_, ver_sum, _) = parse_packet(&bits, 0);
    ver_sum
}

fn calc2(data: &str) -> usize {
    let bits: Vec<char> = data.chars().flat_map(|c| to_binary(c).chars()).collect();
    let (_, _, packet) = parse_packet(&bits, 0);
    packet.eval()
}

fn main() {
    let contnets = fs::read_to_string("input/day16.txt").unwrap();
    let n = calc(&contnets);
    println!("Part1: {}", n);

    let n = calc2(&contnets);
    println!("Part2: {}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(6, calc("D2FE28"));
        assert_eq!(16, calc("8A004A801A8002F478"));
        assert_eq!(12, calc("620080001611562C8802118E34"));
    }

    #[test]
    fn part2_test() {
        assert_eq!(3, calc2("C200B40A82"));
        assert_eq!(54, calc2("04005AC33890"));
        assert_eq!(7, calc2("880086C3E88112"));
        assert_eq!(9, calc2("CE00C43D881120"));
        assert_eq!(1, calc2("D8005AC2A8F0"));
        assert_eq!(0, calc2("F600BC2D8F"));
        assert_eq!(0, calc2("9C005AC2F8F0"));
        assert_eq!(1, calc2("9C0141080250320F1802104A08"));
    }
}
