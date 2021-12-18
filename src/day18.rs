use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum SNum {
    Num(u32),
    Pair(Box<SNum>, Box<SNum>),
}

impl SNum {
    fn reduce(mut self) -> Self {
        loop {
            if self.explode(0).is_some() {
                continue;
            }
            if self.split() {
                continue;
            }
            break;
        }
        self
    }

    fn send_left(&mut self, num: u32) {
        match self {
            SNum::Num(ref mut n) => *n += num,
            SNum::Pair(l, _) => l.send_left(num),
        }
    }

    fn send_right(&mut self, num: u32) {
        match self {
            SNum::Num(ref mut n) => *n += num,
            SNum::Pair(_, r) => r.send_right(num),
        }
    }

    fn explode(&mut self, current_depth: u32) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            SNum::Num(_) => None,
            SNum::Pair(left, right) => {
                if let SNum::Num(l) = **left {
                    if let SNum::Num(r) = **right {
                        if current_depth >= 4 {
                            *self = SNum::Num(0);
                            return Some((Some(l), Some(r)));
                        }
                    }
                }
                if let Some((l, mut r)) = left.explode(current_depth + 1) {
                    if let Some(n) = r {
                        right.send_left(n);
                        r = None;
                    }
                    return Some((l, r));
                }
                if let Some((mut l, r)) = right.explode(current_depth + 1) {
                    if let Some(n) = l {
                        left.send_right(n);
                        l = None;
                    }
                    return Some((l, r));
                }
                None
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            SNum::Num(n) => {
                if *n >= 10 {
                    let l = *n / 2;
                    let r = *n - l;
                    *self = SNum::Pair(Box::new(SNum::Num(l)), Box::new(SNum::Num(r)));
                    true
                } else {
                    false
                }
            }
            SNum::Pair(l, r) => l.split() || r.split(),
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            &SNum::Num(n) => n,
            SNum::Pair(l, r) => (l.magnitude() * 3) + (r.magnitude() * 2),
        }
    }
}

fn parse(input: &mut impl Iterator<Item = char>) -> SNum {
    let mut snums = Vec::new();
    while let Some(c) = input.next() {
        match c {
            '[' => snums.push(parse(input)),
            c @ '0'..='9' => snums.push(SNum::Num(c.to_digit(10).unwrap())),
            ',' => {}
            ']' => {
                return SNum::Pair(
                    Box::new(snums.swap_remove(0)),
                    Box::new(snums.swap_remove(0)),
                )
            }
            _ => unreachable!(),
        }
    }
    snums.swap_remove(0)
}

#[aoc_generator(day18)]
pub fn parse_input(input: &str) -> Vec<SNum> {
    input.lines().map(|l| parse(&mut l.chars())).collect_vec()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &[SNum]) -> u32 {
    let mut numbers = input.iter().cloned();
    let first = numbers.next().unwrap().reduce();
    numbers
        .fold(first, |acc, next| {
            SNum::Pair(Box::new(acc), Box::new(next.reduce())).reduce()
        })
        .magnitude()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[SNum]) -> u32 {
    input
        .iter()
        .tuple_combinations()
        .map(|(n1, n2)| {
            SNum::Pair(Box::new(n1.clone()), Box::new(n2.clone()))
                .reduce()
                .magnitude()
        })
        .max()
        .unwrap()
}
