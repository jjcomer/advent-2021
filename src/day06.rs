use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<usize> {
    let mut fish = vec![0; 9];
    for n in input.split(',').map(|n| n.parse::<usize>().unwrap()) {
        fish[n] += 1
    }
    fish
}

fn reproduce(input: &[usize], days: usize) -> usize {
    let mut fish = input.to_owned();
    for _ in 0..days {
        let new_fish = fish.remove(0);
        fish.push(new_fish);
        fish[6] += new_fish;
    }
    fish.iter().sum()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    reproduce(input, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    reproduce(input, 256)
}
