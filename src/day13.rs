use aoc_runner_derive::{aoc, aoc_generator};
use im::HashSet;

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<(bool, usize)>) {
    let mut parts = input.split("\n\n");
    let points = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut coords = l.split(',');
            (
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let instructions = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            let mut i = l.split('=');
            let direction = i.next().unwrap().ends_with('x');
            (direction, i.next().unwrap().parse().unwrap())
        })
        .collect();
    (points, instructions)
}

fn do_fold(
    sheet: &HashSet<(usize, usize)>,
    (direction, amount): &(bool, usize),
) -> HashSet<(usize, usize)> {
    let mut new_sheet = sheet.clone();
    //true for x
    if *direction {
        for point in sheet.iter().filter(|(x, _)| *x > *amount) {
            new_sheet.remove(point);
            new_sheet.insert((amount - (point.0 - amount), point.1));
        }
    } else {
        for point in sheet.iter().filter(|(_, y)| *y > *amount) {
            new_sheet.remove(point);
            new_sheet.insert((point.0, amount - (point.1 - amount)));
        }
    }

    new_sheet
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(HashSet<(usize, usize)>, Vec<(bool, usize)>)) -> usize {
    let sheet = input.0.clone();
    let instructions = input.1.to_owned();

    do_fold(&sheet, &instructions[0]).len()
}

fn print_sheet(sheet: &HashSet<(usize, usize)>) {
    let max_x = sheet.iter().map(|(x, _)| x).max().unwrap();
    let max_y = sheet.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if sheet.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(HashSet<(usize, usize)>, Vec<(bool, usize)>)) -> usize {
    let sheet = input.0.clone();
    let instructions = input.1.to_owned();

    let final_sheet = instructions
        .iter()
        .fold(sheet, |acc, inst| do_fold(&acc, inst));

    print_sheet(&final_sheet);
    0
}
