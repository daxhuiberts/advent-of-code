use aoctools::mapper;
use aoctools::main;
use aoctools::IterExt;

main!("day01", parse_input);

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .chars()
        .map(mapper!('(' => 1, ')' => -1))
        .scan_ref(0, |state, change| *state += change)
        .collect()
}

fn part1(input: &[i32]) -> i32 {
    input.last().copied().unwrap()
}

fn part2(input: &[i32]) -> usize {
    input.iter().position(|&floor| floor < 0).unwrap() + 1
}

#[test]
fn test_part1() {
    assert_eq!(part1(&parse_input("(())")), 0);
    assert_eq!(part1(&parse_input("()()")), 0);
    assert_eq!(part1(&parse_input("(((")), 3);
    assert_eq!(part1(&parse_input("(()(()(")), 3);
    assert_eq!(part1(&parse_input("))(((((")), 3);
    assert_eq!(part1(&parse_input("())")), -1);
    assert_eq!(part1(&parse_input("))(")), -1);
    assert_eq!(part1(&parse_input(")))")), -3);
    assert_eq!(part1(&parse_input(")())())")), -3);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&parse_input(")")), 1);
    assert_eq!(part2(&parse_input("()())")), 5);
}
