use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(char, char)> {
    input.lines().map(|line|
        line.chars().skip(1).filter(|char| char.is_ascii_uppercase()).collect_tuple().unwrap()
    ).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[(char, char)]) -> String {
    let mut prerequisites: HashMap<char, Vec<char>> = HashMap::new();

    for (before, after) in input {
        prerequisites.entry(*before).or_default();
        prerequisites.entry(*after).or_default().push(*before);
    };

    let mut result = String::new();

    while let Some(char) = prerequisites.iter().filter(|(char, char_prerequisites)| {
        !result.contains(**char) && char_prerequisites.iter().all(|dep_char| result.contains(*dep_char))
    }).map(|x|x.0).sorted().first() {
        result.push(**char);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &'static str = indoc!("
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    ");

    lazy_static! {
        static ref TEST_INPUT_RESULT: Vec<(char, char)> = {
            vec![('C', 'A'), ('C', 'F'), ('A', 'B'), ('A', 'D'), ('B', 'E'), ('D', 'E'), ('F', 'E')]
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), "CABDFE");
    }
}
