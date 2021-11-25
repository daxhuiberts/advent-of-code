static INPUT: &str = include_str!("../../input/day05.txt");

fn main() {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));
}

fn part1(input: &str) -> usize {
    react(input.chars())
}

fn part2(input: &str) -> usize {
    (b'a'..b'z')
        .map(|exclude| {
            react(
                input
                    .chars()
                    .filter(|char| !char.eq_ignore_ascii_case(&(exclude as char))),
            )
        })
        .min()
        .unwrap()
}

fn react(input: impl Iterator<Item = char>) -> usize {
    input
        .fold(vec![], |mut list, right_char| {
            if is_reactive(list.last(), Some(&right_char)) {
                list.pop();
            } else {
                list.push(right_char);
            }

            list
        })
        .len()
}

fn is_reactive(left: Option<&char>, right: Option<&char>) -> bool {
    if let (Some(left), Some(right)) = (left, right) {
        if left.eq_ignore_ascii_case(right) && left != right {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("dabAcCaCBAcCcaDA"), 4);
    }
}
