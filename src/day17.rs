use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .split_terminator(&['.', ',', '='][..])
        .filter_map(|c| c.parse::<i32>().ok())
        .collect_vec()
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    //highest point will hit the lowest point and 0 before -> natural summation of min-y
    (input[2] * (input[2] + 1)) / 2
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Vec<i32>) -> usize {
    let min_x = input[0];
    let max_x = input[1];
    let min_y = input[2];
    let max_y = input[3];
    let mut counter = 0;
    for y_velocity in min_y..=(-min_y) {
        for x_velocity in 0..=max_x {
            let (mut x_velocity, mut y_velocity) = (x_velocity, y_velocity);
            let (mut x_position, mut y_position) = (0, 0);
            while x_position <= max_x && y_position >= min_y {
                if x_position >= min_x && y_position <= max_y {
                    counter += 1;
                    break;
                }
                x_position += x_velocity;
                y_position += y_velocity;
                x_velocity = (x_velocity - 1).max(0);
                y_velocity -= 1;
            }
        }
    }
    counter
}
