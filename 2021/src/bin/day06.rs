use aoctools::main;

main!("day06", parse_input);

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part1(fish: &[u32]) -> usize {
    multiply_fish_many_generations(fish, 80).len()
}

fn part2(_input: &[u32]) -> usize {
    0
}

fn multiply_fish_many_generations(fish: &[u32], generations: usize) -> Vec<u32> {
    (0..generations).fold(fish.to_owned(), |fish, _| multiply_fish(&fish))
}

fn multiply_fish(fish: &[u32]) -> Vec<u32> {
    let ready_fish = fish.iter().filter(|&&fish| fish == 0).count();
    fish.iter()
        .map(|&fish| if fish == 0 { 6 } else { fish - 1 })
        .chain(std::iter::repeat(8).take(ready_fish))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        3,4,3,1,2
    " };

    const INPUT: [u32; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 0);
    }

    #[test]
    fn test_multiply_fish_many_generations() {
        assert_eq!(
            multiply_fish_many_generations(&[3, 4, 3, 1, 2], 18),
            [6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
        );
    }

    #[test]
    fn test_multiply_fish() {
        assert_eq!(multiply_fish(&[2]), [1]);
        assert_eq!(multiply_fish(&[1]), [0]);
        assert_eq!(multiply_fish(&[0]), [6, 8]);
    }
}
