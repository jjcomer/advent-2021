use aoc_runner_derive::{aoc, aoc_generator};

use im::HashMap;
use itertools::Itertools;

type Picture = HashMap<(i32, i32), bool>;

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> (HashMap<(i32, i32), bool>, Vec<bool>) {
    let (rules, raw_picture) = input.split_once("\n\n").unwrap();
    let rules = rules.chars().map(|c| c == '#').collect_vec();
    let mut picture = HashMap::new();
    for (i, l) in raw_picture.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                picture.insert((j as i32, i as i32), true);
            }
        }
    }

    (picture, rules)
}

fn check_rules(
    picture: &Picture,
    rules: &[bool],
    x: i32,
    y: i32,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    cycle: usize,
) -> bool {
    let mut rule_to_check = 0_usize;

    let is_on = |x, y| {
        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            *picture.get(&(x, y)).unwrap_or(&false)
        } else {
            cycle % 2 == 0
        }
    };

    if is_on(x - 1, y - 1) {
        rule_to_check += 256;
    }
    if is_on(x, y - 1) {
        rule_to_check += 128;
    }
    if is_on(x + 1, y - 1) {
        rule_to_check += 64;
    }
    if is_on(x - 1, y) {
        rule_to_check += 32;
    }
    if is_on(x, y) {
        rule_to_check += 16;
    }
    if is_on(x + 1, y) {
        rule_to_check += 8;
    }
    if is_on(x - 1, y + 1) {
        rule_to_check += 4;
    }
    if is_on(x, y + 1) {
        rule_to_check += 2;
    }
    if is_on(x + 1, y + 1) {
        rule_to_check += 1;
    }

    rules[rule_to_check]
}

fn process_picture(picture: &Picture, rules: &[bool], cycle: usize) -> Picture {
    let min_x = picture.keys().map(|k| k.0).min().unwrap();
    let max_x = picture.keys().map(|k| k.0).max().unwrap();
    let min_y = picture.keys().map(|k| k.1).min().unwrap();
    let max_y = picture.keys().map(|k| k.1).max().unwrap();

    let mut new_picture = HashMap::new();

    for i in min_x - 1..=max_x + 1 {
        for j in min_y - 1..=max_y + 1 {
            if check_rules(picture, rules, i, j, min_x, max_x, min_y, max_y, cycle) {
                new_picture.insert((i, j), true);
            }
        }
    }

    new_picture
}

fn solve(picture: &Picture, rules: &[bool], cycles: usize) -> usize {
    let mut picture = picture.clone();
    for cycle in 1..=cycles {
        picture = process_picture(&picture, rules, cycle);
    }

    picture.values().filter(|v| **v).count()
}

#[aoc(day20, part1)]
pub fn solve_part1((picture, rules): &(HashMap<(i32, i32), bool>, Vec<bool>)) -> usize {
    solve(picture, rules, 2)
}

#[aoc(day20, part2)]
pub fn solve_part2((picture, rules): &(HashMap<(i32, i32), bool>, Vec<bool>)) -> usize {
    solve(picture, rules, 50)
}
