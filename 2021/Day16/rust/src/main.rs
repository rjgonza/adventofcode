static INPUT_FILE: &str = include_str!("../../input.txt");

#[derive(Debug, Clone)]
struct LiteralPacket {
    version: u8,
    val: u64,
}

#[derive(Debug, Clone)]
struct OperatorPacket {
    version: u8,
    optype: OpType,
    subpackets: Vec<Packet>,
}

#[derive(Debug, Clone)]
enum Packet {
    Literal(LiteralPacket),
    Operator(OperatorPacket),
}

#[derive(Debug, Clone, Copy)]
enum OpType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

fn read_chunk(bits: &mut dyn Iterator<Item = char>) -> u8 {
    let version_bits: String = bits.take(3).collect();
    u8::from_str_radix(&version_bits, 2).unwrap()
}

fn read_literal(bits: &mut dyn Iterator<Item = char>) -> u64 {
    let mut literal_bits = Vec::new();
    loop {
        let last = bits.next().unwrap() == '0';
        literal_bits.extend(bits.take(4));
        if last {
            let bitstring = literal_bits.iter().collect::<String>();
            return u64::from_str_radix(&bitstring, 2).unwrap();
        }
    }
}

fn read_operator_subpackets(bits: &mut dyn Iterator<Item = char>) -> Vec<Packet> {
    let mut res = Vec::new();

    let length_type_id = bits.next().unwrap();
    match length_type_id {
        '0' => {
            let nbits = usize::from_str_radix(&bits.take(15).collect::<String>(), 2).unwrap();
            let mut subpacket_bits = bits.take(nbits).peekable();
            while subpacket_bits.peek().is_some() {
                res.push(read_packet(&mut subpacket_bits));
            }
        }
        '1' => {
            let nsubpackets = usize::from_str_radix(&bits.take(11).collect::<String>(), 2).unwrap();
            for _ in 0..nsubpackets {
                res.push(read_packet(bits));
            }
        }
        _ => unreachable!(),
    }
    res
}

fn read_packet(bits: &mut dyn Iterator<Item = char>) -> Packet {
    let version = read_chunk(bits);
    let type_id = read_chunk(bits);

    match type_id {
        4 => Packet::Literal(LiteralPacket {
            version,
            val: read_literal(bits),
        }),
        _ => Packet::Operator(OperatorPacket {
            version,
            optype: match type_id {
                0 => OpType::Sum,
                1 => OpType::Product,
                2 => OpType::Min,
                3 => OpType::Max,
                5 => OpType::GreaterThan,
                6 => OpType::LessThan,
                7 => OpType::EqualTo,
                _ => unimplemented!(),
            },
            subpackets: read_operator_subpackets(bits),
        }),
    }
}

fn sum_version(p: &Packet) -> u64 {
    match p {
        Packet::Literal(lit) => lit.version as u64,
        Packet::Operator(op) => {
            let sub_sum = op.subpackets.iter().map(sum_version).sum::<u64>();
            op.version as u64 + sub_sum
        }
    }
}

fn eval(p: &Packet) -> u64 {
    match p {
        Packet::Literal(lit) => lit.val,
        Packet::Operator(op) => {
            let mut sub_vals = op.subpackets.iter().map(eval);
            match op.optype {
                OpType::Sum => sub_vals.sum(),
                OpType::Product => sub_vals.product(),
                OpType::Min => sub_vals.min().unwrap(),
                OpType::Max => sub_vals.max().unwrap(),
                OpType::GreaterThan => {
                    if sub_vals.next().unwrap() > sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                OpType::LessThan => {
                    if sub_vals.next().unwrap() < sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
                OpType::EqualTo => {
                    if sub_vals.next().unwrap() == sub_vals.next().unwrap() {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Part 1: {}", part1(INPUT_FILE));
    println!("Part 2: {}", part2(INPUT_FILE));
}

fn part1(input: &str) -> u64 {
    let mut bits = input
        .chars()
        .flat_map(|char| {
            let val = char.to_digit(16).unwrap();
            vec![
                val & 0b1000u32 != 0,
                val & 0b0100u32 != 0,
                val & 0b0010u32 != 0,
                val & 0b0001u32 != 0,
            ]
        })
        .map(|v| if v { '1' } else { '0' });

    // loop {}

    // dbg!(bits);

    sum_version(&read_packet(&mut bits))
}

fn part2(input: &str) -> u64 {
    let mut bits = input
        .chars()
        .flat_map(|char| {
            let val = char.to_digit(16).unwrap();
            vec![
                val & 0b1000u32 != 0,
                val & 0b0100u32 != 0,
                val & 0b0010u32 != 0,
                val & 0b0001u32 != 0,
            ]
        })
        .map(|v| if v { '1' } else { '0' });

    // loop {}

    // dbg!(bits);

    eval(&read_packet(&mut bits))
}

#[cfg(test)]

mod test {
    use super::*;
    const INPUT1: &str = "8A004A801A8002F478";
    const INPUT2: &str = "620080001611562C8802118E34";
    const INPUT3: &str = "C0015000016115A2E0802F182340";
    const INPUT4: &str = "A0016C880162017C3686B18A3D4780";

    const INPUT5: &str = "C200B40A82";
    const INPUT6: &str = "04005AC33890";
    const INPUT7: &str = "880086C3E88112";
    const INPUT8: &str = "CE00C43D881120";
    const INPUT9: &str = "D8005AC2A8F0";
    const INPUT10: &str = "F600BC2D8F";
    const INPUT11: &str = "9C005AC2F8F0";
    const INPUT12: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT1), 16);
        assert_eq!(part1(INPUT2), 12);
        assert_eq!(part1(INPUT3), 23);
        assert_eq!(part1(INPUT4), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT5), 3);
        assert_eq!(part2(INPUT6), 54);
        assert_eq!(part2(INPUT7), 7);
        assert_eq!(part2(INPUT8), 9);
        assert_eq!(part2(INPUT9), 1);
        assert_eq!(part2(INPUT10), 0);
        assert_eq!(part2(INPUT11), 0);
        assert_eq!(part2(INPUT12), 1);
    }
}
