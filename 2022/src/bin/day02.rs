use aoctools::main;
use itertools::Itertools;

main!("day02", parse_input);

type Input = Vec<(char, char)>;

fn parse_input(input: &'static str) -> Input {
    input.trim().lines().map(|line|
        line.split(" ").map(|s|
            s.chars().next().unwrap()
        ).collect_tuple().unwrap()
    ).collect_vec()
}

fn part1(input: &Input) -> usize {
    input.iter().map(|(opponent, me)| {
        let shape_score = match me {
            'X' => 1,
            'Y' => 2,
            'Z' => 3,
            _ => panic!("should not happen")
        };
        let outcome_score = match (opponent, me) {
            ('A', 'X') => 3,
            ('A', 'Y') => 6,
            ('A', 'Z') => 0,
            ('B', 'X') => 0,
            ('B', 'Y') => 3,
            ('B', 'Z') => 6,
            ('C', 'X') => 6,
            ('C', 'Y') => 0,
            ('C', 'Z') => 3,
            _ => panic!("should not happen")
        };
        shape_score + outcome_score
    }).sum::<usize>()
}

fn part2(input: &Input) -> usize {
    input.iter().map(|(opponent, me)| {
        let outcome_score = match me {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("should not happen")
        };
        let shape_score = match (opponent, me) {
            ('A', 'X') => 3,
            ('A', 'Y') => 1,
            ('A', 'Z') => 2,
            ('B', 'X') => 1,
            ('B', 'Y') => 2,
            ('B', 'Z') => 3,
            ('C', 'X') => 2,
            ('C', 'Y') => 3,
            ('C', 'Z') => 1,
            _ => panic!("should not happen")
        };
        outcome_score + shape_score
    }).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        A Y
        B X
        C Z
    " };

    lazy_static! {
        static ref INPUT: Input = vec![('A', 'Y'), ('B', 'X'), ('C', 'Z')];
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 12);
    }
}
