use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Default, Clone)]
pub struct Board {
    rows: Vec<Vec<u32>>,
    done: bool,
}

impl Board {
    fn new(raw_board: &str) -> Self {
        let rows = raw_board
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|c| c.parse().unwrap())
                    .collect()
            })
            .collect();

        Board {
            rows,
            ..Default::default()
        }
    }

    fn mark(&mut self, num: u32) -> bool {
        for i in 0..self.rows.len() {
            for j in 0..self.rows[0].len() {
                if self.rows[i][j] == num {
                    self.rows[i][j] = 0;
                    return true;
                }
            }
        }
        false
    }

    fn check_win(&self) -> bool {
        for row in self.rows.iter() {
            if row.iter().all(|x| *x == 0) {
                return true;
            }
        }
        for column in 0..self.rows.len() {
            if self.rows.iter().map(|r| r[column]).sum::<u32>() == 0 {
                return true;
            }
        }
        false
    }

    fn score_board(&self) -> u32 {
        self.rows.iter().map(|r| r.iter().sum::<u32>()).sum()
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.split("\n\n");
    let turns = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards = lines.map(Board::new).collect();

    (turns, boards)
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let mut boards = input.1.clone();
    for turn in input.0.iter() {
        for board in boards.iter_mut() {
            if board.mark(*turn) && board.check_win() {
                return *turn * board.score_board();
            }
        }
    }
    0
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &(Vec<u32>, Vec<Board>)) -> u32 {
    let mut boards = input.1.clone();
    for turn in input.0.iter() {
        let board_count = boards.len();
        for board in boards.iter_mut() {
            if board.mark(*turn) {
                if board.check_win() && board_count == 1 {
                    return *turn * board.score_board();
                } else {
                    board.done = true;
                }
            }
        }
        boards = boards.iter().cloned().filter(|b| !b.done).collect();
    }
    0
}
