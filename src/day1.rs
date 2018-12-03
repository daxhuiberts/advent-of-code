use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    input
        .iter()
        .cycle()
        .scan(0, |state, value| { *state += value; Some(*state) })
        .scan(HashSet::new(), |set, value| Some(set.replace(value)))
        .filter_map(|value| value)
        .next()
        .unwrap()
}
