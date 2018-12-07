use itertools::Itertools;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(char, char)> {
    input.lines().map(|line|
        line.chars().skip(1).filter(|char| char.is_ascii_uppercase()).collect_tuple().unwrap()
    ).collect()
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
}
