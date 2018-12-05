#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    react(input.trim().chars())
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.trim();
    (b'a'..b'z').map(|exclude|
        react(input.chars().filter(|char|
            !char.eq_ignore_ascii_case(&(exclude as char))
        ))
    ).min().unwrap()
}

fn react(input: impl Iterator<Item=char>) -> usize {
    input.fold(vec![], |mut list, right_char| {
        if is_reactive(list.last(), Some(&right_char)) {
            list.pop();
        } else {
            list.push(right_char);
        }

        list
    }).len()
}

fn is_reactive(left: Option<&char>, right: Option<&char>) -> bool {
    if let (Some(left), Some(right)) = (left, right) {
        if left.eq_ignore_ascii_case(right) && left != right {
            return true;
        }
    }

    return false;
}

#[test]
fn test_part1() {
    assert_eq!(part1("dabAcCaCBAcCcaDA\n"), 10);
}

#[test]
fn test_part2() {
    assert_eq!(part2("dabAcCaCBAcCcaDA\n"), 4);
}
