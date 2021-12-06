use aoctools::main;

main!("dayxx", parse_input);

fn parse_input(input: &str) -> &str {
    input
}

fn part1(_input: &str) -> u32 {
    0
}

fn part2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        xx
    " };

    const INPUT: &'static str = "xx\n";

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 0);
    }
}
