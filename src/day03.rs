use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn get_counts(input: &Vec<Vec<u32>>) -> Vec<u32> {
    let first = input.first().unwrap().to_owned();
    input.iter().skip(1).fold(first, |acc, x| {
        let mut next = Vec::new();
        for i in 0..acc.len() {
            next.insert(i, acc[i] + x[i]);
        }
        next
    })
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<u32>>) -> u32 {
    let counts = get_counts(input);
    let threshold = input.len() as u32 / 2;
    let gamma = counts
        .iter()
        .map(|b| if *b > threshold { "1" } else { "0" })
        .join("");
    let epsilon = counts
        .iter()
        .map(|b| if *b < threshold { "1" } else { "0" })
        .join("");
    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

fn get_index_count(input: &Vec<Vec<u32>>, index: usize) -> (u32, u32) {
    let ones = input.iter().map(|x| x.get(index).unwrap()).sum();
    (input.len() as u32 - ones, ones)
}

fn to_decimal(input: &Vec<u32>) -> u32 {
    let i = input.iter().map(|x| x.to_string()).join("");
    u32::from_str_radix(&i, 2).unwrap()
}

fn test_oxygen(zeros: u32, ones: u32) -> u32 {
    if zeros <= ones {
        1
    } else {
        0
    }
}

fn test_co2(zeros: u32, ones: u32) -> u32 {
    if zeros >= ones {
        1
    } else {
        0
    }
}

fn find_number(input: &Vec<Vec<u32>>, test: fn(u32, u32) -> u32) -> u32 {
    let mut numbers = input.to_owned();
    for i in 0..input.len() {
        if numbers.len() == 1 {
            return to_decimal(&numbers[0]);
        }
        let (zeros, ones) = get_index_count(&numbers, i);
        let target = test(zeros, ones);
        numbers = numbers.iter().filter(|x| x[i] == target).cloned().collect();
    }
    to_decimal(&numbers[0])
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<u32>>) -> u32 {
    let oxygen = find_number(&input, test_oxygen);
    let co2 = find_number(&input, test_co2);
    oxygen * co2
}
