use aoc_runner_derive::{aoc, aoc_generator};
use im::{vector, Vector};

use std::collections::HashMap;

type Caves = HashMap<String, Pathways>;

#[derive(Debug, Default)]
pub struct Pathways {
    uppers: Vec<String>,
    lowers: Vec<String>,
    start: bool,
    end: bool,
}

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Caves {
    let mut caves: Caves = HashMap::new();
    for l in input.lines() {
        let mut segments = l.trim().split("-");
        let a = segments.next().unwrap().to_string();
        let b = segments.next().unwrap().to_string();

        let mut a_entry = caves.entry(a.clone()).or_default();
        if b.to_lowercase() == b {
            a_entry.lowers.push(b.clone());
        } else {
            a_entry.uppers.push(b.clone());
        };
        match a.as_str() {
            "start" => a_entry.start = true,
            "end" => a_entry.end = true,
            _ => (),
        };

        let mut b_entry = caves.entry(b.clone()).or_default();
        if a.to_lowercase() == a {
            b_entry.lowers.push(a.clone());
        } else {
            b_entry.uppers.push(a.clone());
        }
        match b.as_str() {
            "start" => b_entry.start = true,
            "end" => b_entry.end = true,
            _ => (),
        };
    }

    caves
}

fn fnd_path(
    caves: &Caves,
    current_location: &str,
    current_path: Vector<String>,
    double_lower: bool,
) -> Vector<Vector<String>> {
    let mut possible_paths = Vector::new();

    let mut next_path = current_path.clone();
    next_path.push_back(current_location.to_string());

    let current_pathway = caves.get(current_location).unwrap();
    if current_pathway.end {
        return vector![next_path];
    }

    for next_location in current_pathway.uppers.iter() {
        let new_paths = fnd_path(caves, next_location, next_path.clone(), double_lower);
        for p in new_paths {
            possible_paths.push_back(p);
        }
    }

    if double_lower {
        for next_location in current_pathway
            .lowers
            .iter()
            .filter(|p| !current_path.contains(p))
        {
            let new_paths = fnd_path(caves, next_location, next_path.clone(), double_lower);
            for p in new_paths {
                possible_paths.push_back(p);
            }
        }
    } else {
        for next_location in current_pathway.lowers.iter() {
            if next_location == "start" {
                continue;
            }

            if current_path.contains(next_location) {
                let new_paths = fnd_path(caves, next_location, next_path.clone(), true);
                for p in new_paths {
                    possible_paths.push_back(p);
                }
            } else {
                let new_paths = fnd_path(caves, next_location, next_path.clone(), double_lower);
                for p in new_paths {
                    possible_paths.push_back(p);
                }
            }
        }
    }

    possible_paths
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &Caves) -> usize {
    fnd_path(input, &"start", Vector::new(), true).len()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Caves) -> usize {
    fnd_path(input, &"start", Vector::new(), false).len()
}
