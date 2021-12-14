use aoctools::main;

main!("dayxx", parse_input);

type Input = (&'static str,);

fn parse_input(input: &'static str) -> Input {
    (input.trim(),)
}

fn part1(_input: &Input) -> usize {
    0
}

fn part2(_input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        xx
    " };

    lazy_static! {
        static ref INPUT: Input = ("xx",);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 0);
    }
}
