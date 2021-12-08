use aoctools::main;
use aoctools::IterExt;
use itertools::Itertools;

main!("day08", parse_input);

type Line<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            line.split(" | ")
                .map(|part| part.split_whitespace().collect())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|(_, digits)| {
            digits
                .iter()
                .filter(|digit| [2, 3, 4, 7].contains(&digit.len()))
                .count()
        })
        .sum()
}

fn part2(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|(check_digits, digits)| {
            let digit_map = process_digits(check_digits);
            digits
                .iter()
                .map(|digit| {
                    digit_map
                        .iter()
                        .position(|entry| {
                            let a: Vec<char> = entry.iter().sorted().cloned().collect_vec();
                            let b: Vec<char> = digit.chars().sorted().collect();
                            a == b
                        })
                        .unwrap()
                })
                .fold(0, |number, digit| number * 10 + digit)
        })
        .sum()
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;
const D: usize = 3;
const E: usize = 4;
const F: usize = 5;
const G: usize = 6;

fn process_digits(digits: &[&str]) -> Vec<Vec<char>> {
    let mut digits: Vec<Vec<char>> = digits.iter().map(|digit| digit.chars().collect()).collect();
    let segments: Vec<Vec<char>> = std::iter::repeat("abcdefg")
        .map(|digit| digit.chars().collect())
        .take(7)
        .collect();
    digits.sort_by_key(|digit| if digit.len() == 5 { 99 } else { digit.len() });
    let s = digits
        .iter()
        .fold_ref(segments, |segments, digit| {
            match digit.len() {
                2 => {
                    let segment_c_f = digit;
                    segments[A].retain(|x| !segment_c_f.contains(x));
                    segments[B].retain(|x| !segment_c_f.contains(x));
                    segments[C].retain(|x| segment_c_f.contains(x));
                    segments[D].retain(|x| !segment_c_f.contains(x));
                    segments[E].retain(|x| !segment_c_f.contains(x));
                    segments[F].retain(|x| segment_c_f.contains(x));
                    segments[G].retain(|x| !segment_c_f.contains(x));
                }
                3 => {
                    let segment_a: Vec<_> = digit
                        .iter()
                        .filter(|s| !segments[C].contains(s))
                        .copied()
                        .collect();
                    segments[A].retain(|x| segment_a.contains(x));
                    segments[B].retain(|x| !segment_a.contains(x));
                    segments[C].retain(|x| !segment_a.contains(x));
                    segments[D].retain(|x| !segment_a.contains(x));
                    segments[E].retain(|x| !segment_a.contains(x));
                    segments[F].retain(|x| !segment_a.contains(x));
                    segments[G].retain(|x| !segment_a.contains(x));
                }
                4 => {
                    let segment_b_d: Vec<_> = digit
                        .iter()
                        .filter(|s| !segments[C].contains(s))
                        .copied()
                        .collect();
                    segments[A].retain(|x| !segment_b_d.contains(x));
                    segments[B].retain(|x| segment_b_d.contains(x));
                    segments[C].retain(|x| !segment_b_d.contains(x));
                    segments[D].retain(|x| segment_b_d.contains(x));
                    segments[E].retain(|x| !segment_b_d.contains(x));
                    segments[F].retain(|x| !segment_b_d.contains(x));
                    segments[G].retain(|x| !segment_b_d.contains(x));
                }
                6 => {
                    let absent_segment: Vec<_> =
                        "abcdefg".chars().filter(|x| !digit.contains(x)).collect();
                    segments[A].retain(|x| !absent_segment.contains(x));
                    segments[B].retain(|x| !absent_segment.contains(x));
                    if segments[C].iter().contains(absent_segment.first().unwrap()) {
                        segments[C].retain(|x| absent_segment.contains(x));
                    }
                    if segments[D].iter().contains(absent_segment.first().unwrap()) {
                        segments[D].retain(|x| absent_segment.contains(x));
                    }
                    if segments[E].iter().contains(absent_segment.first().unwrap()) {
                        segments[E].retain(|x| absent_segment.contains(x));
                    }
                    segments[F].retain(|x| !absent_segment.contains(x));
                    segments[G].retain(|x| !absent_segment.contains(x));
                }
                _ => {}
            };
        })
        .into_iter()
        .map(|mut x| x.pop().unwrap())
        .collect_vec();

    vec![
        vec![s[A], s[B], s[C], s[E], s[F], s[G]],       // 0
        vec![s[C], s[F]],                               // 1
        vec![s[A], s[C], s[D], s[E], s[G]],             // 2
        vec![s[A], s[C], s[D], s[F], s[G]],             // 3
        vec![s[B], s[C], s[D], s[F]],                   // 4
        vec![s[A], s[B], s[D], s[F], s[G]],             // 5
        vec![s[A], s[B], s[D], s[E], s[F], s[G]],       // 6
        vec![s[A], s[C], s[F]],                         // 7
        vec![s[A], s[B], s[C], s[D], s[E], s[F], s[G]], // 8
        vec![s[A], s[B], s[C], s[D], s[F], s[G]],       // 9
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    " };

    lazy_static! {
        static ref INPUT: Vec<Line<'static>> = {
            vec![
                (
                    vec![
                        "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                        "fabcd", "edb",
                    ],
                    vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"],
                ),
                (
                    vec![
                        "edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde",
                        "gfcbed", "gfec",
                    ],
                    vec!["fcgedb", "cgb", "dgebacf", "gc"],
                ),
                (
                    vec![
                        "fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac",
                        "gcb", "cdgabef",
                    ],
                    vec!["cg", "cg", "fdcagb", "cbg"],
                ),
                (
                    vec![
                        "fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab",
                        "fgdeca", "fcdbega",
                    ],
                    vec!["efabcd", "cedba", "gadfec", "cb"],
                ),
                (
                    vec![
                        "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb",
                        "dgceab", "fcbdga",
                    ],
                    vec!["gecf", "egdcabf", "bgf", "bfgea"],
                ),
                (
                    vec![
                        "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                        "bafgc", "acf",
                    ],
                    vec!["gebdcfa", "ecba", "ca", "fadegcb"],
                ),
                (
                    vec![
                        "dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb",
                        "gdcebf", "gf",
                    ],
                    vec!["cefg", "dcbef", "fcge", "gbcadfe"],
                ),
                (
                    vec![
                        "bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced",
                        "adcbefg", "gebcd",
                    ],
                    vec!["ed", "bcgafe", "cdgba", "cbgef"],
                ),
                (
                    vec![
                        "egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab",
                        "egfdb", "bfceg",
                    ],
                    vec!["gbdfcae", "bgc", "cg", "cgb"],
                ),
                (
                    vec![
                        "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge",
                        "fdbac", "fegbdc",
                    ],
                    vec!["fgae", "cfgab", "fg", "bagce"],
                ),
            ]
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 61229);
    }
}
