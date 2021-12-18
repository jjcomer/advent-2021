use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day16)]
pub fn parse_input(input: &str) -> Vec<char> {
    input
        .chars()
        .map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<char>>()
        })
        .flatten()
        .collect()
}

fn to_decimal(input: &[char]) -> usize {
    usize::from_str_radix(&String::from_iter(input), 2).unwrap()
}

fn process_packet_p1(input: &[char]) -> (usize, usize) {
    let version = to_decimal(&input[0..3]);
    let operator = to_decimal(&input[3..6]);

    //println!("{} {} {:?}", version, operator, input);

    if operator == 4 {
        let digits = input[6..].chunks(5).take_while(|x| x[0] == '1').count() + 1;
        return (version, (digits * 5) + 6);
    }
    match input[6] {
        '1' => {
            let num_packets = to_decimal(&input[7..18]);
            let mut length = 0;
            let mut versions = 0;
            for _ in 0..num_packets {
                let (p_version, p_length) = process_packet_p1(&input[18 + length..]);
                length += p_length;
                versions += p_version;
            }
            (versions + version, length + 11 + 7)
        }
        '0' => {
            let total_packet_size = to_decimal(&input[7..22]);
            let mut versions = 0;
            let mut consumed_length = 0;
            while consumed_length < total_packet_size {
                let (p_version, p_length) = process_packet_p1(&input[22 + consumed_length..]);
                versions += p_version;
                consumed_length += p_length;
            }
            (versions + version, total_packet_size + 15 + 7)
        }
        _ => unreachable!(),
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &[char]) -> usize {
    let (version, _) = process_packet_p1(input);
    version
}

fn process_packet_p2(input: &[char]) -> (usize, usize) {
    let version = to_decimal(&input[0..3]);
    let operator = to_decimal(&input[3..6]);

    //println!("{} {} {:?}", version, operator, input);

    if operator == 4 {
        let digit_chunks = input[6..]
            .chunks(5)
            .take_while(|x| x[0] == '1')
            .map(|c| c[1..].iter().collect_vec())
            .collect_vec();
        let almost_last = 6 + (digit_chunks.len() * 5) + 1;

        let mut digits = digit_chunks
            .iter()
            .flatten()
            .cloned()
            .cloned()
            .collect_vec();
        for d in input[almost_last..almost_last + 4].iter() {
            digits.push(*d);
        }

        let number = to_decimal(&digits);

        // println!(
        //     "CONSTANT: {} {:?} {:?}",
        //     number,
        //     digits,
        //     &input[6..almost_last + 4]
        // );

        return (number, 6 + ((digit_chunks.len() + 1) * 5));
    }
    let (children, length) = match input[6] {
        '1' => {
            let num_packets = to_decimal(&input[7..18]);
            let mut results = Vec::new();
            let mut length = 0;
            for _ in 0..num_packets {
                let (p_result, p_length) = process_packet_p2(&input[18 + length..]);
                length += p_length;
                results.push(p_result);
            }
            (results, length + 11)
        }
        '0' => {
            let total_packet_size = to_decimal(&input[7..22]);
            let mut results = Vec::new();
            let mut consumed_length = 0;
            while consumed_length < total_packet_size {
                let (p_result, p_length) = process_packet_p2(&input[22 + consumed_length..]);
                results.push(p_result);
                consumed_length += p_length;
            }
            (results, total_packet_size + 15)
        }
        _ => unreachable!(),
    };

    let total_length = length + 7;

    // println!("{} {:?}", operator, children);

    match operator {
        0 => (children.iter().sum(), total_length),
        1 => (children.iter().product(), total_length),
        2 => (*children.iter().min().unwrap(), total_length),
        3 => (*children.iter().max().unwrap(), total_length),
        5 => {
            let result = if children[0] > children[1] { 1 } else { 0 };
            (result, total_length)
        }
        6 => {
            let result = if children[0] < children[1] { 1 } else { 0 };
            (result, total_length)
        }
        7 => {
            let result = if children[0] == children[1] { 1 } else { 0 };
            (result, total_length)
        }
        _ => unreachable!(),
    }
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &[char]) -> usize {
    let (result, _) = process_packet_p2(input);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let samples = vec![
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];
        for (s, a) in samples {
            assert_eq!(a, solve_part1(&parse_input(s)));
        }
    }

    #[test]
    fn generator() {
        let samples = vec![("D2FE28", "110100101111111000101000")];

        for (s, a) in samples {
            assert_eq!(a, String::from_iter(parse_input(s)))
        }
    }

    #[test]
    fn part2() {
        let samples = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for (s, a) in samples {
            assert_eq!(a, solve_part2(&parse_input(s)));
        }
    }
}
