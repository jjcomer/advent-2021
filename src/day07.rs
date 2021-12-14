use std::ops::Div;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<i32> {
    let mut result = input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i32>>();
    result.sort_unstable();
    result
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..=*max)
        .map(|n| input.iter().map(|c| (c - n).abs()).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part1, stats)]
pub fn solve_part1_stats(input: &Vec<i32>) -> i32 {
    let distance = (input[500] + input[499]) / 2;
    input.iter().map(|d| (distance - d).abs()).sum()
}

fn compute_fuel(distance: i32) -> i32 {
    (distance * (distance + 1)) / 2
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();

    (*min..=*max)
        .map(|n| input.iter().map(|c| compute_fuel((c - n).abs())).sum())
        .min()
        .unwrap()
}

#[aoc(day7, part2, stats)]
pub fn solve_part2_stats(input: &Vec<i32>) -> i32 {
    let distance = input.iter().sum::<i32>().div(input.len() as i32);
    input
        .iter()
        .map(|d| compute_fuel((distance - d).abs()))
        .sum()
}
