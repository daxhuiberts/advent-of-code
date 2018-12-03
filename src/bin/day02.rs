extern crate aoc18;
extern crate itertools;

use aoc18::IterExt;
use itertools::Itertools;

fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let list: Vec<&str> = input.lines().collect();

    let result = checksum(list.clone());
    println!("{}", result);

    let result = common_letters(list);
    println!("{}", result);
}

fn checksum(list: Vec<&str>) -> u32 {
    let (twos, threes) = list.iter().map(|entry| {
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

fn common_letters(list: Vec<&str>) -> String {
    list.iter().tuple_combinations().filter_map(|(a, b)| {
        let (matching, differing): (Vec<_>, Vec<_>) = a.chars().zip(b.chars()).partition(|(aa, bb)| aa == bb);
        if differing.len() == 1 {
            Some(matching.iter().fold_ref(String::new(), |string, (aa, _bb)| string.push(*aa)))
        } else {
            None
        }
    }).next().unwrap()
}

#[test]
fn test_checksum() {
    let list = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
    assert_eq!(checksum(list), 12);
}

#[test]
fn test_common_letters() {
    let list = vec!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
    assert_eq!(common_letters(list), "fgij");
}
