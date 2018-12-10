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
    let iterator = std::iter::once(&0).chain(input.iter().cycle());
    let cumulated = iterator.scan(0, |state, value| {
        *state += value;
        Some(*state)
    });
    let duplicates = cumulated.scan(HashSet::new(), |set, value| Some(set.replace(value)));
    duplicates.filter_map(|value| value).next().unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1(&[1, -2, 3, 1]), 3);
    assert_eq!(part1(&[1, 1, 1]), 3);
    assert_eq!(part1(&[1, 1, -2]), 0);
    assert_eq!(part1(&[-1, -2, -3]), -6);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&[1, -1]), 0);
    assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
    assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
    assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
}
