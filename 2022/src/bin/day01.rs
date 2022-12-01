use aoctools::main;
use itertools::Itertools;

main!("day01", parse_input);

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.split("\n\n").map(|block|
        block.trim_end().split("\n").map(|line|
            line.parse().unwrap()
        ).collect()
    ).collect()
}

fn part1(input: &[Vec<u32>]) -> u32 {
    input.iter().map(|group| {
        group.iter().sum()
    }).max().unwrap()
}

fn part2(input: &[Vec<u32>]) -> u32 {
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

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(INPUT_STR)), 24_000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(INPUT_STR)), 45_000);
    }
}
