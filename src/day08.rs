use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

type Digit = HashSet<char>;

#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)> {
    input
        .lines()
        .map(|l| {
            let mut line = l.split(" | ");
            (
                line.next()
                    .unwrap()
                    .split(' ')
                    .map(|s| s.chars().collect())
                    .collect(),
                line.next()
                    .unwrap()
                    .split(' ')
                    .map(|s| s.chars().collect())
                    .collect(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<(Vec<HashSet<char>>, Vec<HashSet<char>>)>) -> usize {
    input
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|s| matches!(s.len(), 2 | 4 | 3 | 7))
                .count()
        })
        .sum()
}

fn six_numbers<'a>(
    one: &Digit,
    four: &Digit,
    unknown: &'a [Digit],
) -> (&'a Digit, &'a Digit, &'a Digit) {
    //0,6,9
    let mut sixes = unknown.iter().filter(|n| n.len() == 6);
    let a = sixes.next().unwrap();
    let b = sixes.next().unwrap();
    let c = sixes.next().unwrap();

    if four.is_subset(a) {
        //a is nine
        if one.is_subset(b) {
            // b is zero
            (b, c, a)
        } else {
            // c is zero
            (c, b, a)
        }
    } else if four.is_subset(b) {
        //b is nine
        if one.is_subset(a) {
            // a is zero
            (a, c, b)
        } else {
            // c is zero
            (c, a, b)
        }
    } else {
        //c is nine
        if one.is_subset(a) {
            // a is zero
            (a, b, c)
        } else {
            // b is zero
            (b, a, c)
        }
    }
}

fn five_numbers<'a>(
    seven: &Digit,
    nine: &Digit,
    numbers: &'a [Digit],
) -> (&'a Digit, &'a Digit, &'a Digit) {
    // 2,3,5
    let mut fives = numbers.iter().filter(|d| d.len() == 5);
    let a = fives.next().unwrap();
    let b = fives.next().unwrap();
    let c = fives.next().unwrap();

    if seven.is_subset(a) {
        //a is 3
        if nine.intersection(b).count() == 5 {
            //b is 5
            (c, a, b)
        } else {
            //c is 5
            (b, a, c)
        }
    } else if seven.is_subset(b) {
        //b is 3
        if nine.intersection(a).count() == 5 {
            //a is 5
            (c, b, a)
        } else {
            //c is 5
            (a, b, c)
        }
    } else {
        //c is 3
        if nine.intersection(a).count() == 5 {
            //a is 5
            (b, c, a)
        } else {
            //c is 5
            (a, c, b)
        }
    }
}

fn map_numbers(unknown: &[Digit], input: &[Digit]) -> u32 {
    let one = unknown.iter().find(|n| n.len() == 2).unwrap();
    let seven = unknown.iter().find(|n| n.len() == 3).unwrap();
    let four = unknown.iter().find(|n| n.len() == 4).unwrap();
    let eight = unknown.iter().find(|n| n.len() == 7).unwrap();
    let (zero, six, nine) = six_numbers(one, four, unknown);
    let (two, three, five) = five_numbers(seven, nine, unknown);

    input
        .iter()
        .map(|d| {
            if d == one {
                "1"
            } else if d == two {
                "2"
            } else if d == three {
                "3"
            } else if d == four {
                "4"
            } else if d == five {
                "5"
            } else if d == six {
                "6"
            } else if d == seven {
                "7"
            } else if d == eight {
                "8"
            } else if d == nine {
                "9"
            } else if d == zero {
                "0"
            } else {
                panic!("UNKNOWN DIGIT {:?}", d)
            }
        })
        .join("")
        .parse()
        .unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<(Vec<Digit>, Vec<Digit>)>) -> u32 {
    input
        .iter()
        .map(|(nums, output)| map_numbers(nums, output))
        .sum()
}
