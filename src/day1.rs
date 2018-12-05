#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    floor_iterator(input).last().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    floor_iterator(input).enumerate().skip_while(|(_position, floor)| *floor >= 0).next().unwrap().0 + 1
}

fn floor_iterator<'a>(input: &'a str) -> impl Iterator<Item=i32> + 'a {
    input.trim().chars().map(|char|
        if char == '(' { 1 } else { -1 }
    ).scan(0, |state, change| {
        *state = *state + change;
        Some(*state)
    })
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

#[test]
fn test_part2() {
    assert_eq!(part2(")"), 1);
    assert_eq!(part2("()())"), 5);
}
