#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    input.trim().chars().fold(vec![], |mut list, right_char| {
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
