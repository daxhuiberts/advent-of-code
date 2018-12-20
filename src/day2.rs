use aoctools::IterExt;
use itertools::Itertools;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let (twos, threes): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|entry| {
            let char_count = entry.chars().group_count();
            let count_count = char_count.values().group_count();
            (count_count.contains_key(&2), count_count.contains_key(&3))
        })
        .unzip();

    twos.iter().filter(|two| **two).count() * threes.iter().filter(|three| **three).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
    input
        .lines()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let matching = a
                .chars()
                .zip_eq(b.chars())
                .filter(|(aa, bb)| aa == bb)
                .collect_vec();
            if matching.len() == a.len() - 1 {
                Some(
                    matching
                        .iter()
                        .fold_ref(String::new(), |string, (aa, _bb)| string.push(*aa)),
                )
            } else {
                None
            }
        })
        .next()
        .unwrap()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = [
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]
        .join("\n");
        assert_eq!(part1(&input), 12);
    }

    #[test]
    fn test_part2() {
        let input = [
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ]
        .join("\n");
        assert_eq!(part2(&input), "fgij");
    }
}
