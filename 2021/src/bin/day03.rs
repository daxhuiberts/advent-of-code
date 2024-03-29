use aoctools::main;

main!("day03", parse_input);

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
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
        .collect()
}

fn part1<T: AsRef<[bool]>>(input: &[T]) -> u32 {
    let common_bits = common_bits(input);
    let gamma_rate = calculate_rate(&common_bits, false);
    let epsilon_rate = calculate_rate(&common_bits, true);
    gamma_rate * epsilon_rate
}

fn calculate_rate(common_bits: &[bool], negate: bool) -> u32 {
    common_bits.iter().fold(0, |number, &common_bit| {
        (number << 1) + (common_bit ^ !negate) as u32
    })
}

fn common_bits<T: AsRef<[bool]>>(input: &[T]) -> Vec<bool> {
    (0..input[0].as_ref().len())
        .map(|column| {
            let count = input.iter().filter(|inner| inner.as_ref()[column]).count();
            count * 2 >= input.len()
        })
        .collect()
}

fn part2<T: AsRef<[bool]> + Clone>(input: &[T]) -> u32 {
    let oxygen_generator_rating = calculate_life_support_rating(input, usize::ge);
    let co2_scrubber_rating = calculate_life_support_rating(input, usize::lt);
    oxygen_generator_rating * co2_scrubber_rating
}

fn calculate_life_support_rating<T: AsRef<[bool]> + Clone>(
    input: &[T],
    comparator: fn(&usize, &usize) -> bool,
) -> u32 {
    let input = input.to_vec();
    let len = input[0].as_ref().len();
    let result: Vec<_> = (0..len).fold(input, |input, column| {
        if input.len() > 1 {
            let count = input.iter().filter(|inner| inner.as_ref()[column]).count();
            let bit_value = comparator(&(count * 2), &input.len());
            input
                .into_iter()
                .filter(|inner| inner.as_ref()[column] == bit_value)
                .collect()
        } else {
            input
        }
    });

    assert_eq!(result.len(), 1);

    result[0]
        .as_ref()
        .iter()
        .fold(0, |number, &bit| (number << 1) + bit as u32)
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

    const INPUT: [[bool; 5]; 12] = [
        [false, false, true, false, false],
        [true, true, true, true, false],
        [true, false, true, true, false],
        [true, false, true, true, true],
        [true, false, true, false, true],
        [false, true, true, true, true],
        [false, false, true, true, true],
        [true, true, true, false, false],
        [true, false, false, false, false],
        [true, true, false, false, true],
        [false, false, false, true, false],
        [false, true, false, true, false],
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
    fn test_part2() {
        assert_eq!(part2(&INPUT), 230)
    }

    #[test]
    fn test_rate() {
        assert_eq!(calculate_rate(&common_bits(&INPUT), true), 22);
        assert_eq!(calculate_rate(&common_bits(&INPUT), false), 9);
    }

    #[test]
    fn test_common_bits() {
        assert_eq!(common_bits(&INPUT), [true, false, true, true, false]);
    }

    #[test]
    fn test_calculate_life_support_rating() {
        assert_eq!(calculate_life_support_rating(&INPUT, usize::ge), 23);
        assert_eq!(calculate_life_support_rating(&INPUT, usize::lt), 10);
    }
}
