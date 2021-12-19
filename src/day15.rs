use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use std::collections::HashSet;

//https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct State {
    cost: i32,
    position: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day15)]
pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn fill_grid(base_grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut a_costs = vec![vec![i32::MAX; base_grid[0].len()]; base_grid.len()];
    let mut points = BinaryHeap::new();
    let mut visited = HashSet::new();
    a_costs[0][0] = 0;
    points.push(State {
        cost: 0,
        position: (0, 0),
    });

    let max_y = base_grid.len() as i32;
    let max_x = base_grid[0].len() as i32;

    while let Some(s) = points.pop() {
        let x = s.position.0;
        let y = s.position.1;
        if visited.contains(&(x, y)) {
            continue;
        }
        let this_cost = a_costs[y as usize][x as usize];
        for (dx, dy) in iproduct!(-1..=1_i32, -1..=1_i32) {
            if dx.abs() == dy.abs()
                || x + dx < 0
                || x + dx >= max_x
                || y + dy < 0
                || y + dy >= max_y
                || visited.contains(&(x + dx, y + dy))
            {
                continue;
            }
            let current_cost = a_costs[(y + dy) as usize][(x + dx) as usize];
            let possible_low = this_cost + base_grid[(y + dy) as usize][(x + dx) as usize];
            if possible_low < current_cost {
                a_costs[(y + dy) as usize][(x + dx) as usize] = possible_low;
                points.push(State {
                    cost: possible_low,
                    position: (dx + x, dy + y),
                });
            }
        }
        visited.insert((x, y));
    }

    a_costs
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> i32 {
    let costs = fill_grid(input);
    costs[input.len() - 1][input[0].len() - 1]
}

fn build_bigger_map(input: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut new_grid = Vec::new();
    for i in 0..5 {
        for y in 0..input.len() {
            let mut y_vec = Vec::new();
            for j in 0..5 {
                for x in 0..input[0].len() {
                    y_vec.push(((input[y][x] + i + j - 1) % 9) + 1)
                }
            }
            new_grid.push(y_vec)
        }
    }
    new_grid
}

fn _print_grid(input: &[Vec<i32>]) {
    for line in input.iter() {
        for x in line {
            print!("{}", x);
        }
        println!();
    }
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> i32 {
    let new_map = build_bigger_map(input);
    //print_grid(&new_map);
    let costs = fill_grid(&new_map);
    costs[new_map.len() - 1][new_map[0].len() - 1]
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1() {
        let input = parse_input(SAMPLE_1);

        assert_eq!(40, solve_part1(&input))
    }

    #[test]
    fn part2() {
        let input = parse_input(SAMPLE_1);
        assert_eq!(315, solve_part2(&input))
    }
}
