use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use std::collections::VecDeque;

type Grid = Vec<Vec<Octo>>;

#[derive(Debug, Default, Clone)]
pub struct Octo {
    last_flash: usize,
    current_energy: usize,
    x: usize,
    y: usize,
}

impl Octo {
    pub fn new(current_energy: usize, x: usize, y: usize) -> Self {
        Octo {
            current_energy,
            x,
            y,
            last_flash: 0,
        }
    }

    fn receive_flash(&mut self, iteration: usize) -> bool {
        if self.last_flash != iteration {
            self.current_energy += 1;
            if self.current_energy > 9 {
                self.last_flash = iteration;
                self.current_energy = 0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn get_neighbours(ox: usize, oy: usize) -> Vec<(usize, usize)> {
    iproduct!(
        (ox as i32 - 1)..=(ox as i32 + 1),
        (oy as i32 - 1)..=(oy as i32 + 1)
    )
    .filter(|(x, y)| {
        *x >= 0 && *x < 10 && *y >= 0 && *y < 10 && !(ox as i32 == *x && oy as i32 == *y)
    })
    .map(|(x, y)| (x as usize, y as usize))
    .collect_vec()
}

fn process_cycle(grid: &mut Grid, cycle: usize) -> usize {
    //add one to everyone
    for l in grid.iter_mut() {
        for o in l.iter_mut() {
            o.current_energy += 1;
        }
    }

    let mut flashes = 0;
    let mut to_flash = grid
        .iter()
        .flatten()
        .filter(|o| o.current_energy > 9)
        .map(|o| (o.x, o.y))
        .collect::<VecDeque<(usize, usize)>>();

    while let Some((x, y)) = to_flash.pop_front() {
        let o = grid.get_mut(y).unwrap().get_mut(x).unwrap();
        if o.receive_flash(cycle) {
            flashes += 1;
            for n in get_neighbours(x, y) {
                to_flash.push_back(n);
            }
        }
    }

    flashes
}

#[aoc_generator(day11)]
pub fn parse_input(input: &str) -> Vec<Vec<Octo>> {
    input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .map(|(x, c)| Octo::new(c.to_digit(10).unwrap() as usize, x, y))
                .collect()
        })
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Grid) -> usize {
    let mut grid = input.to_owned();
    let mut flashes = 0;

    for cycle in 1..=100_usize {
        let new_flashes = process_cycle(&mut grid, cycle);
        flashes += new_flashes;
        //print_grid(cycle, &grid);
    }

    flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Grid) -> usize {
    let mut grid = input.to_owned();
    let target = grid.len() * grid[0].len();
    let mut cycle = 0;
    loop {
        cycle += 1;
        let new_flashes = process_cycle(&mut grid, cycle);
        if new_flashes == target {
            return cycle;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = parse_input(
            "5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526",
        );

        assert_eq!(1656, solve_part1(&input))
    }
}
