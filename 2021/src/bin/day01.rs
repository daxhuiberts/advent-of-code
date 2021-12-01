use aoctools::main;
use itertools::Itertools;

main!("day01", parse_input);

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &[usize]) -> usize {
    count_increases(input)
}

fn part2(input: &[usize]) -> usize {
    let aggregate = input.windows(3).map(|w| w.iter().sum()).collect_vec();
    count_increases(&aggregate)
}

fn count_increases(input: &[usize]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 5);
    }
}
