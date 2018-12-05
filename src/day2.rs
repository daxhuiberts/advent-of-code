use itertools::Itertools;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines().map(|line|
        line.split('x').map(|x| x.parse().unwrap()).collect_tuple().unwrap()
    ).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[(u32, u32, u32)]) -> u32 {
    input.iter().map(|lwh| wrapping_paper(*lwh)).sum()
}

fn wrapping_paper((length, width, height): (u32, u32, u32)) -> u32 {
    let (lw, lh, wh) = (length * width, length * height, width * height);
    lw * 2 + lh * 2 + wh * 2 + std::cmp::min(std::cmp::min(lw, lh), wh)
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
fn test_wrapping_paper() {
    assert_eq!(wrapping_paper((2, 3, 4)), 58);
    assert_eq!(wrapping_paper((1, 1, 10)), 43);
}
