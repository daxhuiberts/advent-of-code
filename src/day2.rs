use super::IterExt;
use super::itertools::Itertools;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let (twos, threes) = input.lines().map(|entry| {
        let char_count = entry.chars().group_count();
        let count_count = char_count.values().group_count();
        (count_count.contains_key(&2), count_count.contains_key(&3))
    }).fold((0, 0), |(mut twos, mut threes), (two, three)| {
        if two { twos += 1 }
        if three { threes += 1 }
        (twos, threes)
    });

    twos * threes
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    input.lines().tuple_combinations().filter_map(|(a, b)| {
        let (matching, differing): (Vec<_>, Vec<_>) = a.chars().zip(b.chars()).partition(|(aa, bb)| aa == bb);
        if differing.len() == 1 {
            Some(matching.iter().fold_ref(String::new(), |string, (aa, _bb)| string.push(*aa)))
        } else {
            None
        }
    }).next().unwrap()
}

#[test]
fn test_part1() {
    let input = ["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"].join("\n");
    assert_eq!(part1(&input), 12);
}

#[test]
fn test_part2() {
    let input = ["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"].join("\n");
    assert_eq!(part2(&input), "fgij");
}
