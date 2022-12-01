use aoctools::main;
use itertools::Itertools;
use std::ops::Deref;

main!("day01", parse_input);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.split("\n\n").map(|block|
        block.trim_end().split("\n").map(|line|
            line.parse().unwrap()
        ).collect()
    ).collect()
}

fn part1<T: Deref<Target = [U]>, U: Deref<Target = [u32]>>(input: T) -> u32 {
    input.iter().map(|group| {
        group.iter().sum()
    }).max().unwrap()
}

fn part2<T: Deref<Target = [U]>, U: Deref<Target = [u32]>>(input: T) -> u32 {
    input.iter().map(|group| {
        group.iter().sum::<u32>()
    }).sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    " };

    fn input() -> Vec<Vec<u32>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = input();
        let input = input.iter().map(|group| group.deref()).collect_vec();
        let input: &[&[u32]] = &input;
        assert_eq!(parse_input(INPUT_STR), input);
    }

    #[test]
    fn test_part1() {
        let input = input();
        let input = input.iter().map(|group| group.deref()).collect_vec();
        let input: &[&[u32]] = &input;
        assert_eq!(part1(input), 24_000);
    }

    #[test]
    fn test_part2() {
        let input = input();
        let input = input.iter().map(|group| group.deref()).collect_vec();
        let input: &[&[u32]] = &input;
        assert_eq!(part2(input), 45_000);
    }
}
