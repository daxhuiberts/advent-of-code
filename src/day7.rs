use itertools::Itertools;

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input.lines().map(|line|
        line.bytes().skip(1).filter(|char| char.is_ascii_uppercase()).collect_tuple().unwrap()
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
        static ref TEST_INPUT_RESULT: Vec<(u8, u8)> = {
            vec![(b'C', b'A'), (b'C', b'F'), (b'A', b'B'), (b'A', b'D'), (b'B', b'E'), (b'D', b'E'), (b'F', b'E')]
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT), *TEST_INPUT_RESULT);
    }
}
