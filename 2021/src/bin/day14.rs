use aoctools::main;
use aoctools::IterExt;
use itertools::Itertools;
use std::collections::HashMap;

main!("day14", parse_input);

type Input = (&'static str, HashMap<(char, char), char>);

fn parse_input(input: &'static str) -> Input {
    let (template, rules) = input.split("\n\n").collect_tuple().unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (left, right) = line.split(" -> ").collect_tuple().unwrap();
            let left = left.chars().collect_tuple().unwrap();
            (left, right.chars().next().unwrap())
        })
        .collect();
    (template, rules)
}

fn part1(input: &Input) -> usize {
    calculate_polymer(input, 10)
}

fn part2(input: &Input) -> usize {
    calculate_polymer(input, 40)
}

fn calculate_polymer((template, rules): &Input, iterations: usize) -> usize {
    let map = template
        .chars()
        .tuple_windows()
        .fold_ref(HashMap::new(), |map, (left, right)| {
            *map.entry((left, right)).or_insert(0) += 1
        });

    let final_map = (0..iterations).fold(map, |map, _| {
        map.iter()
            .flat_map(|(&(left, right), &count)| {
                let new_char = rules.get(&(left, right)).unwrap();
                [((left, *new_char), count), ((*new_char, right), count)]
            })
            .fold_ref(HashMap::new(), |map, (entry, count)| {
                *map.entry(entry).or_insert(0) += count
            })
    });

    let counts: HashMap<char, usize> = final_map.iter().fold_ref(
        HashMap::from([(template.chars().last().unwrap(), 1)]),
        |map, ((left, _), count)| *map.entry(*left).or_insert(0) += count,
    );

    let (min, max) = counts.values().cloned().minmax().into_option().unwrap();

    max - min
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C
    " };

    lazy_static! {
        static ref INPUT: Input = (
            "NNCB",
            HashMap::from([
                (('C', 'H'), 'B'),
                (('H', 'H'), 'N'),
                (('C', 'B'), 'H'),
                (('N', 'H'), 'C'),
                (('H', 'B'), 'C'),
                (('H', 'C'), 'B'),
                (('H', 'N'), 'C'),
                (('N', 'N'), 'C'),
                (('B', 'H'), 'H'),
                (('N', 'C'), 'B'),
                (('N', 'B'), 'B'),
                (('B', 'N'), 'B'),
                (('B', 'B'), 'N'),
                (('B', 'C'), 'B'),
                (('C', 'C'), 'N'),
                (('C', 'N'), 'C'),
            ])
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 1588);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 2188189693529);
    }
}
