use aoctools::main;

main!("day07", parse_input);

fn parse_input(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(crabs: &[i32]) -> i32 {
    calculate_ideal_position(crabs, std::convert::identity)
}

fn part2(crabs: &[i32]) -> i32 {
    calculate_ideal_position(crabs, triangulate)
}

fn calculate_ideal_position(crabs: &[i32], calculator: fn(i32) -> i32) -> i32 {
    let (min, max) = min_max(crabs);
    (min..=max)
        .map(|position| calculate_for_position(crabs, position, calculator))
        .min()
        .unwrap()
}

fn calculate_for_position(crabs: &[i32], position: i32, calculator: fn(i32) -> i32) -> i32 {
    crabs
        .iter()
        .map(|crab| calculator((position - crab).abs()))
        .sum()
}

fn min_max<T: Ord + Copy>(input: &[T]) -> (T, T) {
    (
        input.iter().min().copied().unwrap(),
        input.iter().max().copied().unwrap(),
    )
}

fn triangulate(value: i32) -> i32 {
    value * (value + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        16,1,2,0,4,2,7,1,2,14
    " };

    const INPUT: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 37);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 168);
    }

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&INPUT), (0, 16));
    }

    #[test]
    fn test_triangulate() {
        assert_eq!(triangulate(1), 1);
        assert_eq!(triangulate(2), 3);
        assert_eq!(triangulate(3), 6);
        assert_eq!(triangulate(4), 10);
        assert_eq!(triangulate(5), 15);
    }
}
