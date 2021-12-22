use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

type Instruction = (bool, i64, i64, i64, i64, i64, i64);

#[aoc_generator(day22)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    let digits_regex = Regex::new(r"(-?\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let (on_off, coords) = l.split_once(' ').unwrap();
            let digits = digits_regex
                .captures_iter(coords)
                .map(|m| m[1].parse::<i64>().unwrap())
                .collect_vec();
            (
                on_off == "on",
                digits[0],
                digits[1],
                digits[2],
                digits[3],
                digits[4],
                digits[5],
            )
        })
        .collect_vec()
}

fn p1(input: &[Instruction]) -> usize {
    let mut cubes = HashSet::new();

    for &(on_off, x0, x1, y0, y1, z0, z1) in input.iter() {
        if x0 > 50 || x1 < -50 || y0 > 50 || y1 < -50 || z0 > 50 || z1 < -50 {
            continue;
        }
        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    if on_off {
                        cubes.insert((x, y, z));
                    } else {
                        cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    cubes.len()
}

#[aoc(day22, part1)]
pub fn solve_p1(input: &[Instruction]) -> usize {
    p1(input)
}

#[derive(Debug, Default, Clone)]
struct Cube {
    on_off: bool,
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
}

impl Cube {
    fn intersection(&self, other: &Cube) -> Option<Cube> {
        let x0 = i64::max(self.x0, other.x0);
        let x1 = i64::min(self.x1, other.x1);
        let y0 = i64::max(self.y0, other.y0);
        let y1 = i64::min(self.y1, other.y1);
        let z0 = i64::max(self.z0, other.z0);
        let z1 = i64::min(self.z1, other.z1);

        if x0 <= x1 && y0 <= y1 && z0 <= z1 {
            Some(Cube {
                x0,
                x1,
                y0,
                y1,
                z0,
                z1,
                ..Default::default()
            })
        } else {
            None
        }
    }

    fn volume(&self) -> i64 {
        let sign = if self.on_off { 1 } else { -1 };
        ((self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1) * (self.z1 - self.z0 + 1)) * sign
    }
}

fn p2(input: &[Instruction]) -> i64 {
    let mut cubes: Vec<Cube> = Vec::new();
    for &(on_off, x0, x1, y0, y1, z0, z1) in input.iter() {
        let new_cube = Cube {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
            on_off,
        };
        let mut cubes_to_add = Vec::new();
        if on_off {
            cubes_to_add.push(new_cube.clone());
        }
        for cube in cubes.iter() {
            if let Some(mut intersecting_cube) = new_cube.intersection(cube) {
                intersecting_cube.on_off = !cube.on_off;
                cubes_to_add.push(intersecting_cube);
            }
        }
        cubes.extend(cubes_to_add);
    }

    cubes.iter().map(|c| c.volume()).sum()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &[Instruction]) -> i64 {
    p2(input)
}
