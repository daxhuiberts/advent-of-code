#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input.trim().chars().map(|char| if char == '(' { 1 } else { -1 }).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("(())"), 0);
    assert_eq!(part1("()()"), 0);
    assert_eq!(part1("((("), 3);
    assert_eq!(part1("(()(()("), 3);
    assert_eq!(part1("))((((("), 3);
    assert_eq!(part1("())"), -1);
    assert_eq!(part1("))("), -1);
    assert_eq!(part1(")))"), -3);
    assert_eq!(part1(")())())"), -3);
}
