use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day02.txt");
    let list = input.lines().collect();
    let result = checksum(list);
    println!("{:?}", result);
}

fn checksum(list: Vec<&str>) -> u32 {
    let (twos, threes) = list.iter().map(|entry| {
        let char_count = entry.chars().fold(HashMap::new(), |mut map, char| {
            *map.entry(char).or_insert(0) += 1;
            map
        });
        let count_count = char_count.iter().fold(HashMap::new(), |mut map, (_char, count)| {
            *map.entry(count).or_insert(0) += 1;
            map
        });
        (count_count.contains_key(&2), count_count.contains_key(&3))
    }).fold((0, 0), |(mut twos, mut threes), (two, three)| {
        if two { twos += 1 }
        if three { threes += 1 }
        (twos, threes)
    });

    twos * threes
}

#[test]
fn test_checksum() {
    let list = vec!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
    assert_eq!(checksum(list), 12);
}
