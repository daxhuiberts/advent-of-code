use itertools::Itertools;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<(i32, i32)> {
    input.lines().map(|line|
        line.split(", ").map(|coordinate| coordinate.parse().unwrap()).collect_tuple().unwrap()
    ).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n";
    const PARSED: [(i32, i32); 6] = [(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), PARSED);
    }
}
