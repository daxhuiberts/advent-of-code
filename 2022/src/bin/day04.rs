use std::ops::RangeInclusive;
use aoctools::main;
use itertools::Itertools;

main!("day04", parse_input);

type Input = Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>;

fn parse_input(input: &'static str) -> Input {
    input.trim().lines().map(|line| {
        line.split(",").map(|range| {
            let (begin, end) = range.split("-").map(|number|
                number.parse().unwrap()
            ).collect_tuple().unwrap();
            RangeInclusive::new(begin, end)
        }).collect_tuple().unwrap()
    }).collect()
}

fn part1(input: &Input) -> usize {
    input.iter().filter(|(left, right)| {
        (left.contains(right.start()) && left.contains(right.end())) ||
        (right.contains(left.start()) && right.contains(left.end()))
    }).count()
}

fn part2(input: &Input) -> usize {
    input.iter().filter(|(left, right)| {
        left.contains(right.start()) || left.contains(right.end()) ||
        right.contains(left.start()) || right.contains(left.end())
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    " };

    lazy_static! {
        static ref INPUT: Input = vec![
            (RangeInclusive::new(2, 4), RangeInclusive::new(6, 8)),
            (RangeInclusive::new(2, 3), RangeInclusive::new(4, 5)),
            (RangeInclusive::new(5, 7), RangeInclusive::new(7, 9)),
            (RangeInclusive::new(2, 8), RangeInclusive::new(3, 7)),
            (RangeInclusive::new(6, 6), RangeInclusive::new(4, 6)),
            (RangeInclusive::new(2, 6), RangeInclusive::new(4, 8)),
        ];
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 4);
    }
}
