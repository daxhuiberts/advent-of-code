use itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("../../input/day07.txt");

fn main() {
    let input = parse_input(INPUT);
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Box<HashMap<char, Vec<char>>> {
    Box::new(parse_input_step2(&parse_input_step1(input)))
}

fn parse_input_step1(input: &str) -> Vec<(char, char)> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .skip(1)
                .filter(|char| char.is_ascii_uppercase())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn parse_input_step2(input: &[(char, char)]) -> HashMap<char, Vec<char>> {
    let mut prerequisites: HashMap<char, Vec<char>> = HashMap::new();

    for (before, after) in input {
        prerequisites.entry(*before).or_default();
        prerequisites.entry(*after).or_default().push(*before);
    }

    prerequisites
}

fn part1(prerequisites: &HashMap<char, Vec<char>>) -> String {
    let mut result = String::new();

    while let Some(char) = get_next(prerequisites, &result, "") {
        result.push(char);
    }

    result
}

fn part2(prerequisites: &HashMap<char, Vec<char>>) -> usize {
    part2_inner(prerequisites, 5, 60)
}

fn part2_inner(
    prerequisites: &HashMap<char, Vec<char>>,
    worker_count: usize,
    delay: usize,
) -> usize {
    let mut workers = vec![None; worker_count];
    let mut exclude = String::new();
    let mut result = String::new();
    let mut seconds = 0;

    loop {
        for worker in &mut workers {
            if let None = *worker {
                *worker = get_next(prerequisites, &result, &exclude).map(|char| {
                    exclude.push(char);
                    (char, char as usize - 64 + delay)
                })
            }
        }

        if workers.iter().all(|worker| worker.is_none()) {
            return seconds;
        }

        for worker in &mut workers {
            if let Some((char, sec)) = *worker {
                if sec == 1 {
                    result.push(char);
                    *worker = None;
                } else {
                    *worker = Some((char, sec - 1));
                }
            }
        }

        seconds += 1;
    }
}

fn get_next(prerequisites: &HashMap<char, Vec<char>>, done: &str, exclude: &str) -> Option<char> {
    prerequisites
        .iter()
        .filter(|(char, char_prerequisites)| {
            !done.contains(**char)
                && !exclude.contains(**char)
                && char_prerequisites
                    .iter()
                    .all(|dep_char| done.contains(*dep_char))
        })
        .map(|x| *x.0)
        .sorted()
        .next()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const TEST_INPUT: &'static str = indoc!(
        "
        Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.
    "
    );

    lazy_static! {
        static ref TEST_INPUT_INTERMEDIATE_RESULT: Vec<(char, char)> = {
            vec![
                ('C', 'A'),
                ('C', 'F'),
                ('A', 'B'),
                ('A', 'D'),
                ('B', 'E'),
                ('D', 'E'),
                ('F', 'E'),
            ]
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
        assert_eq!(
            parse_input_step1(TEST_INPUT),
            *TEST_INPUT_INTERMEDIATE_RESULT
        );
    }

    #[test]
    fn test_parse_input_step2() {
        assert_eq!(
            parse_input_step2(&*TEST_INPUT_INTERMEDIATE_RESULT),
            *TEST_INPUT_RESULT
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), "CABDFE");
    }

    #[test]
    fn test_part2_inner() {
        assert_eq!(part2_inner(&*TEST_INPUT_RESULT, 2, 0), 15);
    }
}
