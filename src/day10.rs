use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn find_corrupted(line: &[char]) -> (usize, Vec<char>) {
    let mut parens = Vec::new();
    for c in line {
        match c {
            '(' | '[' | '{' | '<' => parens.push(*c),
            ')' => {
                if parens.pop().unwrap() != '(' {
                    return (3, parens);
                }
            }
            ']' => {
                if parens.pop().unwrap() != '[' {
                    return (57, parens);
                }
            }
            '}' => {
                if parens.pop().unwrap() != '{' {
                    return (1197, parens);
                }
            }
            '>' => {
                if parens.pop().unwrap() != '<' {
                    return (25137, parens);
                }
            }
            _ => unreachable!(),
        };
    }
    (0, parens)
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Vec<Vec<char>>) -> usize {
    input.iter().map(|l| find_corrupted(l).0).sum()
}

fn close_chunk(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
}

fn gen_closing_score(open_chunks: &[char]) -> usize {
    open_chunks
        .iter()
        .rev()
        .fold(0, |acc, c| (acc * 5) + close_chunk(*c))
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Vec<Vec<char>>) -> usize {
    let answers = input
        .iter()
        .map(|l| find_corrupted(l))
        .filter(|(score, _)| *score == 0)
        .map(|(_, open_chunks)| gen_closing_score(&open_chunks))
        .sorted()
        .collect::<Vec<usize>>();

    answers[(answers.len() / 2)]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let input = vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "(((({<>}<{<{<>}{[]{[]{}",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .iter()
        .map(|l| l.chars().collect_vec())
        .collect_vec();
        assert_eq!(288957, solve_part2(&input))
    }
}
