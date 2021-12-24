use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Registry(usize),
    Value(isize),
}

impl From<&str> for Source {
    fn from(input: &str) -> Self {
        match input {
            "w" => Source::Registry(0),
            "x" => Source::Registry(1),
            "y" => Source::Registry(2),
            "z" => Source::Registry(3),
            _ => Source::Value(input.parse().unwrap()),
        }
    }
}

impl Source {
    fn get_value(&self, registries: &[isize]) -> isize {
        match *self {
            Self::Registry(i) => registries[i],
            Self::Value(value) => value,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Input(usize),
    Add(usize, Source),
    Mul(usize, Source),
    Div(usize, Source),
    Mod(usize, Source),
    Eql(usize, Source),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let registry = match &input[4..5] {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => unreachable!(),
        };

        match &input[0..3] {
            "inp" => Self::Input(registry),
            "add" => Self::Add(registry, input[6..].into()),
            "mul" => Self::Mul(registry, input[6..].into()),
            "div" => Self::Div(registry, input[6..].into()),
            "mod" => Self::Mod(registry, input[6..].into()),
            "eql" => Self::Eql(registry, input[6..].into()),
            _ => unreachable!(),
        }
    }
}

#[aoc_generator(day24)]
pub fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    let instructions: Vec<Instruction> = input.lines().map(|l| l.into()).collect_vec();
    instructions
        .chunks(18)
        .map(|block| block.iter().copied().collect_vec())
        .collect_vec()
}

fn find_model_number(
    cache: &mut HashMap<(isize, usize), Option<isize>>,
    blocks: &[Vec<Instruction>],
    block: usize,
    z: isize,
    valid_numbers: &[isize],
) -> Option<isize> {
    if let Some(&result) = cache.get(&(z, block)) {
        return result;
    }

    for &d in valid_numbers {
        let mut registries = [d, 0, 0, z];
        for instruction in blocks[block].iter() {
            match *instruction {
                Instruction::Add(r, s) => registries[r] += s.get_value(&registries),
                Instruction::Mul(r, s) => registries[r] *= s.get_value(&registries),
                Instruction::Div(r, s) => registries[r] /= s.get_value(&registries),
                Instruction::Mod(r, s) => registries[r] %= s.get_value(&registries),
                Instruction::Eql(r, s) => {
                    registries[r] = (registries[r] == s.get_value(&registries)) as isize
                }
                Instruction::Input(_) => continue,
            }
        }
        let z = registries[3];

        if block == blocks.len() - 1 {
            if z == 0 {
                cache.insert((z, block), Some(d));
                return Some(d);
            }
            continue;
        }
        if let Some(result) = find_model_number(cache, blocks, block + 1, z, valid_numbers) {
            let num = (result * 10) + d;
            cache.insert((z, block), Some(num));
            return Some(num);
        }
    }
    cache.insert((z, block), None);
    None
}

pub fn p1(input: &[Vec<Instruction>]) -> String {
    let valid_numbers = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let biggest = find_model_number(&mut HashMap::new(), input, 0, 0, &valid_numbers).unwrap();

    biggest.to_string().chars().rev().collect()
}

#[aoc(day24, part1)]
pub fn solve_p1(input: &[Vec<Instruction>]) -> String {
    p1(input)
}

pub fn p2(input: &[Vec<Instruction>]) -> String {
    let valid_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let smallest = find_model_number(&mut HashMap::new(), input, 0, 0, &valid_numbers).unwrap();

    smallest.to_string().chars().rev().collect()
}

#[aoc(day24, part2)]
pub fn solve_p2(input: &[Vec<Instruction>]) -> String {
    p2(input)
}
