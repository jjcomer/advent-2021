use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn pares_input(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|l| {
            let mut s = l.split(' ');
            (
                s.next().unwrap().to_owned(),
                s.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<(String, i32)>) -> i32 {
    let mut depth = 0;
    let mut position = 0;

    for (action, x) in input {
        match action.as_str() {
            "forward" => position += x,
            "down" => depth += x,
            "up" => depth -= x,
            _ => panic!("Unknown direction {}", action),
        }
    }

    depth * position
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<(String, i32)>) -> i32 {
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;

    for (action, x) in input {
        match action.as_str() {
            "forward" => {
                position += x;
                depth += x * aim
            }
            "down" => aim += x,
            "up" => aim -= x,

            _ => panic!("Unknown direction {}", action),
        }
    }

    depth * position
}
