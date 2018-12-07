use std::collections::HashMap;
use itertools::Itertools;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Box<HashMap<char, Vec<char>>> {
    Box::new(parse_input_step2(&parse_input_step1(input)))
}

pub fn parse_input_step1(input: &str) -> Vec<(char, char)> {
    input.lines().map(|line|
        line.chars().skip(1).filter(|char| char.is_ascii_uppercase()).collect_tuple().unwrap()
    ).collect()
}

pub fn parse_input_step2(input: &[(char, char)]) -> HashMap<char, Vec<char>> {
    let mut prerequisites: HashMap<char, Vec<char>> = HashMap::new();

    for (before, after) in input {
        prerequisites.entry(*before).or_default();
        prerequisites.entry(*after).or_default().push(*before);
    };

    prerequisites
}

#[aoc(day7, part1)]
pub fn part1(prerequisites: &HashMap<char, Vec<char>>) -> String {
    let mut result = String::new();

    while let Some(char) = get_next(prerequisites, &result) {
        result.push(char);
    }

    result
}

fn get_next(prerequisites: &HashMap<char, Vec<char>>, done: &str) -> Option<char> {
    prerequisites.iter().filter(|(char, char_prerequisites)| {
        !done.contains(**char) && char_prerequisites.iter().all(|dep_char| done.contains(*dep_char))
    }).map(|x|*x.0).sorted().first().cloned()
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
        static ref TEST_INPUT_INTERMEDIATE_RESULT: Vec<(char, char)> = {
            vec![('C', 'A'), ('C', 'F'), ('A', 'B'), ('A', 'D'), ('B', 'E'), ('D', 'E'), ('F', 'E')]
        };

        static ref TEST_INPUT_RESULT: HashMap<char, Vec<char>> = {
            let mut map = HashMap::new();
            map.insert('A', vec!['C']);
            map.insert('B', vec!['A']);
            map.insert('C', vec![]);
            map.insert('D', vec!['A']);
            map.insert('E', vec!['B', 'D', 'F']);
            map.insert('F', vec!['C']);
            map
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT), Box::new(TEST_INPUT_RESULT.clone()));
    }

    #[test]
    fn test_parse_input_step1() {
        assert_eq!(parse_input_step1(TEST_INPUT), *TEST_INPUT_INTERMEDIATE_RESULT);
    }

    #[test]
    fn test_parse_input_step2() {
        assert_eq!(parse_input_step2(&*TEST_INPUT_INTERMEDIATE_RESULT), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), "CABDFE");
    }
}
