#[aoc_generator(day8)]
pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|item| item.parse().unwrap())
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[usize]) -> usize {
    let mut iter = input.iter().cloned();
    let entry = parse_entry(&mut iter);
    entry.sum_meta()
}

#[aoc(day8, part2)]
pub fn part2(input: &[usize]) -> usize {
    let mut iter = input.iter().cloned();
    let entry = parse_entry(&mut iter);
    entry.node_value()
}

struct Entry {
    children: Vec<Entry>,
    meta: Vec<usize>,
}

impl Entry {
    fn sum_meta(&self) -> usize {
        let children_sum = self
            .children
            .iter()
            .map(|child| child.sum_meta())
            .sum::<usize>();
        let meta_sum = self.meta.iter().sum::<usize>();

        children_sum + meta_sum
    }

    fn node_value(&self) -> usize {
        if self.children.is_empty() {
            self.meta.iter().sum()
        } else {
            self.meta
                .iter()
                .filter_map(|index| self.children.get(index - 1))
                .map(|child| child.node_value())
                .sum()
        }
    }
}

fn parse_entry(input: &mut Iterator<Item = usize>) -> Entry {
    let children_count = input.next().unwrap();
    let meta_count = input.next().unwrap();

    let children = (0..children_count).map(|_| parse_entry(input)).collect();
    let meta = input.take(meta_count).collect();

    Entry { children, meta }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;

    const TEST_INPUT: &'static str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    lazy_static! {
        static ref TEST_INPUT_RESULT: Vec<usize> =
            vec![2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*TEST_INPUT_RESULT), 66);
    }
}
