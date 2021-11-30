use aoctools::main;
use itertools::Itertools;

main!("day02", parse_input);

fn parse_input(input: &str) -> Vec<(u32, u32, u32)> {
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|x| x.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part1(input: &[(u32, u32, u32)]) -> u32 {
    input.iter().map(|lwh| wrapping_paper(*lwh)).sum()
}

fn part2(input: &[(u32, u32, u32)]) -> u32 {
    input.iter().map(|lwh| ribbon(*lwh)).sum()
}

fn wrapping_paper((length, width, height): (u32, u32, u32)) -> u32 {
    let (lw, lh, wh) = (length * width, length * height, width * height);
    lw * 2 + lh * 2 + wh * 2 + min3(lw, lh, wh)
}

fn ribbon((length, width, height): (u32, u32, u32)) -> u32 {
    let shortest_side = min3(length + width, length + height, width + height);
    shortest_side * 2 + length * width * height
}

fn min3<T: Ord>(a: T, b: T, c: T) -> T {
    std::cmp::min(std::cmp::min(a, b), c)
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("2x3x4\n1x1x10\n"), vec![(2, 3, 4), (1, 1, 10)]);
}

#[test]
fn test_part1() {
    assert_eq!(part1(&[(2, 3, 4), (1, 1, 10)]), 101);
}

#[test]
fn test_part2() {
    assert_eq!(part2(&[(2, 3, 4), (1, 1, 10)]), 48);
}

#[test]
fn test_wrapping_paper() {
    assert_eq!(wrapping_paper((2, 3, 4)), 58);
    assert_eq!(wrapping_paper((1, 1, 10)), 43);
}

#[test]
fn test_ribbon() {
    assert_eq!(ribbon((2, 3, 4)), 34);
    assert_eq!(ribbon((1, 1, 10)), 14);
}
