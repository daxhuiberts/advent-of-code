use aoctools::main;
use itertools::Itertools;

main!("day05", parse_input);

fn parse_input(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn part1(input: &[&[u8]]) -> usize {
    input.iter().filter(|&line| naughty_or_nice(line)).count()
}

fn part2(_input: &[&[u8]]) -> &'static str {
    "unimplemented"
}

fn naughty_or_nice(input: &[u8]) -> bool {
    let min_three_vowels = input.iter().filter(|char| b"aeiou".contains(char)).count() >= 3;
    let double_letter = input
        .iter()
        .tuple_windows()
        .any(|(left, right)| left == right);
    let disallowed: &[&[u8]] = &[b"ab", b"cd", b"pq", b"xy"];
    let disalowed_pattern = input
        .windows(2)
        .any(|pattern| disallowed.contains(&pattern));

    min_three_vowels && double_letter && !disalowed_pattern
}

#[test]
fn test_naughty_or_nice() {
    assert_eq!(naughty_or_nice(b"aaa"), true);
    assert_eq!(naughty_or_nice(b"ugknbfddgicrmopn"), true);
    assert_eq!(naughty_or_nice(b"jchzalrnumimnmhp"), false);
    assert_eq!(naughty_or_nice(b"haegwjzuvuyypxyu"), false);
    assert_eq!(naughty_or_nice(b"dvszwmarrgswjxmb"), false);
}
