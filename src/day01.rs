#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|s| s.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut last = &i32::MAX;

    for i in input.iter() {
        if i > last {
            counter += 1;
        }
        last = i;
    }

    counter
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut last = i32::MAX;

    for x in input.windows(3) {
        let current = x.iter().sum();
        if current > last {
            counter += 1;
        }
        last = current
    }

    counter
}
