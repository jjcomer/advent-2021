use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.split(",").map(|n| n.parse().unwrap()).collect()
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
