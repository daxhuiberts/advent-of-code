use aoctools::main;
use aoctools::transpose;

main!("day03", parse_input);

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let parsed = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '0' => false,
                    '1' => true,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    transpose(parsed)
}

fn part1<T: AsRef<[bool]>>(input: &[T]) -> u32 {
    let common_bits = common_bits(input);
    let gamma_rate = rate(&common_bits, false);
    let epsilon_rate = rate(&common_bits, true);
    gamma_rate * epsilon_rate
}

fn part2<T: AsRef<[bool]>>(_input: &[T]) -> usize {
    0
}

fn rate(common_bits: &[bool], negate: bool) -> u32 {
    common_bits.iter().fold(0, |number, &common_bit| {
        (number << 1) + if common_bit ^ negate { 1 } else { 0 }
    })
}

fn common_bits<T: AsRef<[bool]>>(input: &[T]) -> Vec<bool> {
    input
        .iter()
        .map(|column| {
            column.as_ref().iter().filter(|&&cell| cell).count() > (column.as_ref().len() / 2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    " };

    const INPUT: [[bool; 12]; 5] = [
        [
            false, true, true, true, true, false, false, true, true, true, false, false,
        ],
        [
            false, true, false, false, false, true, false, true, false, true, false, true,
        ],
        [
            true, true, true, true, true, true, true, true, false, false, false, false,
        ],
        [
            false, true, true, true, false, true, true, false, false, false, true, true,
        ],
        [
            false, false, false, true, true, true, true, false, false, true, false, false,
        ],
    ];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 198)
    }

    #[test]
    fn test_part2() {}

    #[test]
    fn test_rate() {
        assert_eq!(rate(&common_bits(&INPUT), false), 22);
        assert_eq!(rate(&common_bits(&INPUT), true), 9);
    }

    #[test]
    fn test_common_bits() {
        assert_eq!(common_bits(&INPUT), [true, false, true, true, false]);
    }
}
