use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn short_part1(input: &Vec<i32>) -> usize {
    input.windows(2).filter(|x| x[0] < x[1]).count()
}

#[aoc(day1, part2)]
pub fn short_part2(input: &Vec<i32>) -> usize {
    input
        .windows(4)
        .filter(|x| x[0..3].iter().sum::<i32>() < x[1..4].iter().sum())
        .count()
}
