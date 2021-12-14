use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
//use std::collections::HashSet;
use im::HashSet;
type Grid = Vec<Vec<u32>>;

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn check_low_point(grid: &Grid, x: usize, y: usize) -> Option<u32> {
    let current_point = grid[y][x];
    if x != 0 && (current_point >= grid[y][x - 1]) {
        None
    } else if x != 99 && (current_point >= grid[y][x + 1]) {
        None
    } else if y != 0 && (current_point >= grid[y - 1][x]) {
        None
    } else if y != 99 && (current_point >= grid[y + 1][x]) {
        None
    } else {
        Some(current_point)
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Grid) -> u32 {
    let mut points = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if let Some(point) = check_low_point(input, x, y) {
                points.push(point + 1)
            }
        }
    }
    points.iter().sum()
}

fn check_in_basin(basins: &[HashSet<(usize, usize)>], y: usize, x: usize) -> Vec<usize> {
    let neighbours = [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)];
    let mut matching_basins = Vec::new();
    for (i, basin) in basins.iter().enumerate() {
        for n in neighbours {
            if basin.contains(&n) {
                matching_basins.push(i);
                break;
            }
        }
    }
    matching_basins
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Grid) -> usize {
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 9 {
                continue;
            }
            let matching_basins = check_in_basin(&basins, y, x);
            if matching_basins.is_empty() {
                let mut new_basin = HashSet::new();
                new_basin.insert((y, x));
                basins.push(new_basin);
            } else if matching_basins.len() == 1 {
                basins[matching_basins[0]].insert((y, x));
            } else {
                let mut new_basin = HashSet::new();
                for i in matching_basins.iter() {
                    new_basin = new_basin.union(basins[*i].clone());
                }
                new_basin.insert((y, x));
                let mut new_basins = vec![new_basin];
                for (i, x) in basins.iter().enumerate() {
                    if matching_basins.contains(&i) {
                        continue;
                    }
                    new_basins.push(x.clone());
                }
                basins = new_basins;
            }
        }
    }
    basins
        .iter()
        .map(|x| x.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}
