use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct LineSeg {
    a: (i32, i32),
    b: (i32, i32),
}

impl LineSeg {
    fn new(input: &str) -> Option<Self> {
        let mut points = input.split(" -> ");
        let mut raw_a = points.next()?.split(",");
        let mut raw_b = points.next()?.split(",");
        Some(LineSeg {
            a: (
                raw_a.next()?.parse().unwrap(),
                raw_a.next()?.parse().unwrap(),
            ),
            b: (
                raw_b.next()?.parse().unwrap(),
                raw_b.next()?.parse().unwrap(),
            ),
        })
    }

    fn is_straight(&self) -> bool {
        (self.a.0 == self.b.0) || (self.a.1 == self.b.1)
    }

    fn gen_straight_points(&self) -> Vec<(i32, i32)> {
        if self.a.0 == self.b.0 {
            if self.a.1 <= self.b.1 {
                (self.a.1..=self.b.1).map(|i| (self.a.0, i)).collect()
            } else {
                (self.b.1..=self.a.1).map(|i| (self.a.0, i)).collect()
            }
        } else {
            if self.a.0 <= self.b.0 {
                (self.a.0..=self.b.0).map(|i| (i, self.a.1)).collect()
            } else {
                (self.b.0..=self.a.0).map(|i| (i, self.a.1)).collect()
            }
        }
    }

    fn gen_diagonal_points(&self) -> Vec<(i32, i32)> {
        if self.a.0 <= self.b.0 {
            if self.a.1 <= self.b.1 {
                (self.a.0..=self.b.0).zip(self.a.1..=self.b.1).collect()
            } else {
                (self.a.0..=self.b.0)
                    .zip((self.b.1..=self.a.1).rev())
                    .collect()
            }
        } else {
            if self.a.1 <= self.b.1 {
                (self.b.0..=self.a.0)
                    .rev()
                    .zip(self.a.1..=self.b.1)
                    .collect()
            } else {
                (self.b.0..=self.a.0)
                    .rev()
                    .zip((self.b.1..=self.a.1).rev())
                    .collect()
            }
        }
    }

    fn gen_points(&self) -> Vec<(i32, i32)> {
        if self.is_straight() {
            self.gen_straight_points()
        } else {
            self.gen_diagonal_points()
        }
    }
}

#[aoc_generator(day5)]
pub fn parse_input(input: &str) -> Vec<LineSeg> {
    input.lines().map(|l| LineSeg::new(l).unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Vec<LineSeg>) -> usize {
    let mut points = HashMap::new();

    for l in input.iter().filter(|x| x.is_straight()) {
        for p in l.gen_points() {
            let counter = points.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    points.values().filter(|v| **v > 1).count()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Vec<LineSeg>) -> usize {
    let mut points = HashMap::new();

    for l in input.iter() {
        for p in l.gen_points() {
            let counter = points.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    points.values().filter(|v| **v > 1).count()
}
