use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day05.txt");

fn main() {
    println!("part 1: {}", part1(&INPUT));
}

fn part1(input: &str) -> usize {
    input.lines().filter(|line| naughty_or_nice(line)).count()
}

fn naughty_or_nice(input: &str) -> bool {
    let three_vowels = input
        .as_bytes()
        .iter()
        .filter(|char| b"aeiou".contains(char))
        .take(3)
        .count()
        == 3;
    let double_letter = input
        .as_bytes()
        .iter()
        .tuple_windows()
        .any(|(left, right)| left == right);
    let disallowed: &[&[u8]] = &[b"ab", b"cd", b"pq", b"xy"];
    let disalowed_pattern = input
        .as_bytes()
        .windows(2)
        .any(|pattern| disallowed.contains(&pattern));

    three_vowels && double_letter && !disalowed_pattern
}

#[test]
fn test_naughty_or_nice() {
    assert_eq!(naughty_or_nice("aaa"), true);
    assert_eq!(naughty_or_nice("ugknbfddgicrmopn"), true);
    assert_eq!(naughty_or_nice("jchzalrnumimnmhp"), false);
    assert_eq!(naughty_or_nice("haegwjzuvuyypxyu"), false);
    assert_eq!(naughty_or_nice("dvszwmarrgswjxmb"), false);
}
