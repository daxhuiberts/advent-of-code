#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.trim();
    (0..).skip_while(|index| {
        let hash = md5::compute(format!("{}{}", input, index));
        (hash[0] as usize + hash[1] as usize + (hash[2] >> 4) as usize) != 0
    }).next().unwrap()
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.trim();
    (0..).skip_while(|index| {
        let hash = md5::compute(format!("{}{}", input, index));
        (hash[0] as usize + hash[1] as usize + hash[2] as usize) != 0
    }).next().unwrap()
}

// #[test]
// fn test_part1() {
//     assert_eq!(part1("abcdef"), 609043);
//     assert_eq!(part1("pqrstuv"), 1048970);
// }
