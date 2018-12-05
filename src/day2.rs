use itertools::Itertools;

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines().map(|line|
        line.split('x').map(|x| x.parse().unwrap()).collect_tuple().unwrap()
    ).collect()
}

#[test]
fn test_parse_input() {
    assert_eq!(parse_input("2x3x4\n1x1x10\n"), vec![(2, 3, 4), (1, 1, 10)]);
}
