use aoctools::main;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;

main!("day03", parse_input);

type Input = Vec<&'static str>;
type Set = HashSet<char, RandomState>;

fn parse_input(input: &'static str) -> Input {
    input.trim().lines().collect()
}

fn duplicate_item(line: &str) -> char {
    assert!(line.len() % 2 == 0);

    let (left, right) = line.split_at(line.len() / 2);

    let left: Set = HashSet::from_iter(left.chars());
    let right: Set = HashSet::from_iter(right.chars());

    left.intersection(&right).cloned().next().unwrap()
}

fn part1(input: &Input) -> usize {
    let scores: Vec<_> = ('a'..='z').chain('A'..='Z').collect();

    input.iter().map(|line| {
        let duplicate = duplicate_item(line);

        scores.iter().position(|c|*c == duplicate).unwrap() + 1
    }).sum()
}

fn duplicate_group_item((a, b, c): (&&str, &&str, &&str)) -> char {
    let a: Set = HashSet::from_iter(a.chars());
    let b: Set = HashSet::from_iter(b.chars());
    let c: Set = HashSet::from_iter(c.chars());

    let tmp: Set = a.intersection(&b).cloned().collect();
    tmp.intersection(&c).cloned().next().unwrap()
}

fn part2(input: &Input) -> usize {
    let scores: Vec<_> = ('a'..='z').chain('A'..='Z').collect();

    input.iter().tuples().map(|group| {
        let duplicate = duplicate_group_item(group);

        scores.iter().position(|c|*c == duplicate).unwrap() + 1
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    " };

    lazy_static! {
        static ref INPUT: Input = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_parse_duplicate_item() {
        let duplicates = vec!['p', 'L', 'P', 'v', 't', 's'];
        INPUT.iter().zip(duplicates).for_each(|(line, duplicate)| {
            assert_eq!(duplicate_item(line), duplicate);
        });
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 70);
    }
}
