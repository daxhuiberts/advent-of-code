use aoctools::main;
use itertools::Itertools;
use std::collections::HashMap;

main!("day06", parse_input);

fn parse_input(input: &str) -> HashMap<u32, usize> {
    input.trim().split(',').map(|x| x.parse().unwrap()).counts()
}

fn part1(fish: &HashMap<u32, usize>) -> usize {
    multiply_fish_many_generations(fish, 80).values().sum()
}

fn part2(fish: &HashMap<u32, usize>) -> usize {
    multiply_fish_many_generations(fish, 256).values().sum()
}

fn multiply_fish_many_generations(
    fish: &HashMap<u32, usize>,
    generations: usize,
) -> HashMap<u32, usize> {
    (0..generations).fold(fish.to_owned(), |fish, _| multiply_fish(&fish))
}

fn multiply_fish(fish: &HashMap<u32, usize>) -> HashMap<u32, usize> {
    let mut new_fish: HashMap<u32, usize> = fish
        .iter()
        .filter_map(|(&age, &count)| (age != 0).then(|| (age - 1, count)))
        .collect();

    if let Some(&count) = fish.get(&0) {
        *new_fish.entry(6).or_insert(0) += count;
        new_fish.insert(8, count);
    }

    new_fish
    // .chain([(8, *fish.get(&0).unwrap_or(&0))])
    // .collect::<Vec<(u32, usize)>>();
    // new_fish.into_iter().collect()

    // let ready_fish = fish.iter().filter(|&&fish| fish == 0).count();
    // fish.iter()
    //     .map(|&fish| if fish == 0 { 6 } else { fish - 1 })
    //     .chain(std::iter::repeat(8).take(ready_fish))
    //     .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        3,4,3,1,2
    " };

    lazy_static! {
        static ref INPUT: HashMap<u32, usize> = [3, 4, 3, 1, 2].into_iter().counts();
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 5934);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 26984457539);
    }

    #[test]
    fn test_multiply_fish_many_generations() {
        assert_eq!(
            multiply_fish_many_generations(&*INPUT, 18),
            [6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8]
                .into_iter()
                .counts()
        );
    }

    #[test]
    fn test_multiply_fish() {
        assert_eq!(
            multiply_fish(&HashMap::from([(2, 1)])),
            HashMap::from([(1, 1)])
        );
        assert_eq!(
            multiply_fish(&HashMap::from([(1, 1)])),
            HashMap::from([(0, 1)])
        );
        assert_eq!(
            multiply_fish(&HashMap::from([(0, 1)])),
            HashMap::from([(6, 1), (8, 1)])
        );
    }
}
