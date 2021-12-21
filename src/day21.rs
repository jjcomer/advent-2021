use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use itertools::{iproduct, Itertools};
use lazy_static::lazy_static;

#[aoc_generator(day21)]
pub fn parse_input(input: &str) -> (u32, u32) {
    let (p1, p2) = input.split_once('\n').unwrap();
    (
        p1.chars().last().unwrap().to_digit(10).unwrap(),
        p2.chars().last().unwrap().to_digit(10).unwrap(),
    )
}

fn solve_p1(p1: u32, p2: u32) -> usize {
    let mut rolls = 0;
    let mut roll_values = (1..=100).cycle();

    let mut p1_score = 0_usize;
    let mut p2_score = 0_usize;
    let mut p1_location = p1 as usize;
    let mut p2_location = p2 as usize;

    loop {
        let p1_roll: usize =
            roll_values.next().unwrap() + roll_values.next().unwrap() + roll_values.next().unwrap();
        rolls += 3;
        p1_location = ((p1_location + p1_roll - 1) % 10) + 1;
        p1_score += p1_location;

        if p1_score >= 1000 {
            return p2_score * rolls;
        }

        let p2_roll: usize =
            roll_values.next().unwrap() + roll_values.next().unwrap() + roll_values.next().unwrap();

        rolls += 3;
        p2_location = ((p2_location + p2_roll - 1) % 10) + 1;
        p2_score += p2_location;

        if p2_score >= 1000 {
            return p1_score * rolls;
        }
    }
}

lazy_static! {
    static ref ROLLS: Vec<u32> = iproduct!(1..=3, 1..=3, 1..=3)
        .map(|(x, y, z)| x + y + z)
        .collect_vec();
}

#[cached]
fn play_part_2(p1: u32, p2: u32, score_1: u32, score_2: u32, player: usize) -> [usize; 2] {
    let mut wins = [0, 0];
    for roll in ROLLS.iter() {
        let mut positions = [p1, p2];
        let mut score = [score_1, score_2];
        positions[player] = (positions[player] + roll - 1) % 10 + 1;
        score[player] += positions[player];

        if score[player] >= 21 {
            wins[player] += 1;
        } else {
            let new_wins = play_part_2(
                positions[0],
                positions[1],
                score[0],
                score[1],
                (player + 1) % 2,
            );

            wins[0] += new_wins[0];
            wins[1] += new_wins[1];
        }
    }
    wins
}

#[aoc(day21, part1)]
pub fn solve_part1((p1, p2): &(u32, u32)) -> usize {
    solve_p1(*p1, *p2)
}

#[aoc(day21, part2)]
pub fn slove_part2((p1, p2): &(u32, u32)) -> usize {
    let wins = play_part_2(*p1, *p2, 0, 0, 0);
    wins.into_iter().max().unwrap()
}
