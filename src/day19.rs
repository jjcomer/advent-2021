use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

type Point = (i32, i32, i32);

#[aoc_generator(day19)]
pub fn parse_input(input: &str) -> Vec<Vec<Point>> {
    input
        .split("\n\n")
        .map(|scanner| {
            scanner
                .lines()
                .skip(1)
                .map(|l| {
                    let mut coords = l.split(',');
                    (
                        coords.next().unwrap().parse().unwrap(),
                        coords.next().unwrap().parse().unwrap(),
                        coords.next().unwrap().parse().unwrap(),
                    )
                })
                .collect_vec()
        })
        .collect_vec()
}

fn gen_rotation((x, y, z): &Point) -> Vec<Point> {
    let x = *x;
    let y = *y;
    let z = *z;

    vec![
        (x, y, z),
        (y, -x, z),
        (-x, -y, z),
        (-y, x, z),
        (z, y, -x),
        (y, -z, -x),
        (-z, -y, -x),
        (-y, z, -x),
        (z, -x, -y),
        (-x, -z, -y),
        (-z, x, -y),
        (x, z, -y),
        (z, -y, x),
        (-y, -z, x),
        (-z, y, x),
        (y, z, x),
        (z, x, y),
        (x, -z, y),
        (-z, -x, y),
        (-x, z, y),
        (-x, y, -z),
        (y, x, -z),
        (x, -y, -z),
        (-y, -x, -z),
    ]
}

fn join_scan(total_beacons: &mut HashSet<Point>, scanner: &[Point]) -> Option<Point> {
    for (dx, dy, dz) in total_beacons
        .iter()
        .cartesian_product(scanner)
        .map(|((x1, y1, z1), (x2, y2, z2))| (x1 - x2, y1 - y2, z1 - z2))
    {
        let full_distances = scanner.iter().map(|(x, y, z)| (dx + x, dy + y, dz + z));
        if full_distances
            .clone()
            .filter(|beacon| total_beacons.contains(beacon))
            .count()
            >= 12
        {
            total_beacons.extend(full_distances);
            return Some((dx, dy, dz));
        }
    }
    None
}

fn scan(scanners: &[Vec<Point>], total_beacons: &mut HashSet<Point>) -> Option<(usize, Point)> {
    scanners.iter().enumerate().find_map(|(i, scanner)| {
        let rotated_points = scanner.iter().map(gen_rotation).collect_vec();
        (0..24)
            .map(|j| rotated_points.iter().map(|p| p[j]).collect_vec())
            .find_map(|rotation| join_scan(total_beacons, &rotation))
            .map(|distance| (i, distance))
    })
}

fn p1(input: &[Vec<Point>]) -> usize {
    let mut scanners = input.to_owned();
    let mut total_beacons: HashSet<Point> = scanners.swap_remove(0).iter().cloned().collect();

    while !scanners.is_empty() {
        if let Some((i, _distance)) = scan(&scanners, &mut total_beacons) {
            scanners.swap_remove(i);
            continue;
        }
    }
    total_beacons.len()
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &[Vec<Point>]) -> usize {
    p1(input)
}

fn p2(input: &[Vec<Point>]) -> i32 {
    let mut scanners = input.to_owned();
    let mut total_beacons: HashSet<Point> = scanners.swap_remove(0).iter().cloned().collect();
    let mut distances = Vec::new();

    while !scanners.is_empty() {
        if let Some((i, distance)) = scan(&scanners, &mut total_beacons) {
            scanners.swap_remove(i);
            distances.push(distance);
            continue;
        }
    }

    distances
        .iter()
        .tuple_combinations()
        .map(|((x1, y1, z1), (x2, y2, z2))| (x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs())
        .max()
        .unwrap()
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &[Vec<Point>]) -> i32 {
    p2(input)
}
