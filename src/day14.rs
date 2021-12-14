use aoc_runner_derive::{aoc, aoc_generator};
use im::HashMap;
use itertools::Itertools;

type Instructions = HashMap<(char, char), char>;

#[aoc_generator(day14)]
pub fn parse_input(input: &str) -> (Vec<char>, Instructions) {
    let mut sections = input.split("\n\n");
    let components = sections.next().unwrap().chars().collect();
    let mut instructions = HashMap::new();
    for l in sections.next().unwrap().lines() {
        let chars = l.chars().collect_vec();
        instructions.insert((chars[0], chars[1]), chars[6]);
    }
    (components, instructions)
}

fn compute_cycle(
    pairs: &HashMap<(char, char), usize>,
    instructions: &Instructions,
) -> HashMap<(char, char), usize> {
    let mut new_counts = pairs.clone();
    for (k, v) in pairs.iter() {
        if *v > 0 {
            let new_char = instructions.get(k).unwrap();
            *new_counts.entry(*k).or_default() -= v;
            *new_counts.entry((k.0, *new_char)).or_default() += v;
            *new_counts.entry((*new_char, k.1)).or_default() += v;
        }
    }
    new_counts
}

fn get_counts(orig_input: &[char], pairs: &HashMap<(char, char), usize>) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    *counts.entry(*orig_input.last().unwrap()).or_default() += 1;
    for ((c, _), v) in pairs.iter() {
        if *v > 0 {
            *counts.entry(*c).or_default() += v;
        }
    }
    counts
}

fn solve_day(cycles: usize, components: Vec<char>, instructions: Instructions) -> usize {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();
    for (a, b) in components.iter().tuple_windows() {
        *pairs.entry((*a, *b)).or_default() += 1;
    }

    for _ in 0..cycles {
        pairs = compute_cycle(&pairs, &instructions);
    }

    let counts = get_counts(&components, &pairs);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[aoc(day14, part1)]
pub fn solve_part1((components, instructions): &(Vec<char>, Instructions)) -> usize {
    solve_day(10, components.clone(), instructions.clone())
}

#[aoc(day14, part2)]
pub fn solve_part2((components, instructions): &(Vec<char>, Instructions)) -> usize {
    solve_day(40, components.clone(), instructions.clone())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_1: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1() {
        let input = parse_input(SAMPLE_1);

        assert_eq!(1588, solve_part1(&input))
    }

    #[test]
    fn part2() {
        let input = parse_input(SAMPLE_1);
        assert_eq!(2188189693529, solve_part2(&input))
    }
}
