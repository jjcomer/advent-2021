use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cucumber {
    South,
    East,
}

type Map = HashMap<(usize, usize), Cucumber>;

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> (usize, usize, Map) {
    let y = input.lines().count();
    let x = input.lines().next().unwrap().chars().count();
    let mut map = HashMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            match c {
                'v' => map.insert((x, y), Cucumber::South),
                '>' => map.insert((x, y), Cucumber::East),
                _ => None,
            };
        }
    }

    (x, y, map)
}

fn p1(max_x: usize, max_y: usize, map: &Map) -> usize {
    let mut counter = 0;
    let mut old_map = map.clone();

    loop {
        //print_map(&old_map, max_x, max_y, counter);
        let mut moved = false;
        let mut new_map = HashMap::new();
        counter += 1;

        for ((x, y), cucumber) in old_map.iter() {
            match *cucumber {
                Cucumber::East => {
                    let new_coords = (((x + 1) % max_x), *y);
                    if !old_map.contains_key(&new_coords) {
                        //println!("Moving East: {} {} {:?}", x, y, new_coords);
                        moved = true;
                        new_map.insert(new_coords, Cucumber::East);
                    } else {
                        new_map.insert((*x, *y), Cucumber::East);
                    }
                }
                Cucumber::South => {
                    new_map.insert((*x, *y), Cucumber::South);
                }
            };
        }

        old_map = new_map;
        new_map = HashMap::new();

        for ((x, y), cucumber) in old_map.iter() {
            match *cucumber {
                Cucumber::South => {
                    let new_coords = (*x, ((y + 1) % max_y));
                    if !old_map.contains_key(&new_coords) {
                        //println!("Moving South: {} {} {:?}", x, y, new_coords);
                        moved = true;
                        new_map.insert(new_coords, Cucumber::South);
                    } else {
                        new_map.insert((*x, *y), Cucumber::South);
                    }
                }
                Cucumber::East => {
                    new_map.insert((*x, *y), Cucumber::East);
                }
            };
        }

        if !moved {
            break;
        } else {
            old_map = new_map;
        }
    }

    counter
}

fn print_map(map: &Map, max_x: usize, max_y: usize, cycle: usize) {
    println!("\nCycle: {}\n", cycle);
    for y in 0..max_y {
        for x in 0..max_x {
            if let Some(cucumber) = map.get(&(x, y)) {
                match *cucumber {
                    Cucumber::East => print!(">"),
                    Cucumber::South => print!("v"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!()
}

#[aoc(day25, part1)]
pub fn solve_p1((max_x, max_y, map): &(usize, usize, Map)) -> usize {
    p1(*max_x, *max_y, map)
}
//#[aoc(day25, part2)]

#[cfg(test)]
mod test {
    use super::*;

    const sample: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_p1() {
        let (max_x, max_y, map) = parse_input(sample);
        assert_eq!(58, p1(max_x, max_y, &map));
    }
}
