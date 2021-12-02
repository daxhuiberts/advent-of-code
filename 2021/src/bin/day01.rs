use aoctools::main;
use itertools::Itertools;

main!("day01", parse_input);

fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &[u32]) -> usize {
    count_increases(input)
}

fn part2(input: &[u32]) -> usize {
    let aggregate = input.windows(3).map(|w| w.iter().sum()).collect_vec();
    count_increases(&aggregate)
}

fn count_increases(input: &[u32]) -> usize {
    input.iter().tuple_windows().filter(|(a, b)| a < b).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 5);
    }
}
