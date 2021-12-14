use aoctools::main;
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

fn part2(_input: &Input) -> usize {
    0
}

fn calculate_polymer((template, rules): &Input, iterations: usize) -> usize {
    let result = (0..iterations).fold(template.to_string(), |template, _| {
        interleave(&template, rules)
    });
    let (min, max) = result
        .chars()
        .counts()
        .values()
        .cloned()
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

fn interleave(template: &str, rules: &HashMap<(char, char), char>) -> String {
    let insertions = template
        .chars()
        .tuple_windows::<(char, char)>()
        .map(|window| rules.get(&window).unwrap().clone());
    template.chars().by_ref().interleave(insertions).collect()
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
        assert_eq!(part2(&*INPUT), 0);
    }
}
