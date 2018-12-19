use linked_list::{LinkedList, Cursor};

#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Box<(usize, usize)> {
    let mut iter = input.split_whitespace();
    let players = iter.next().unwrap().parse().unwrap();
    let last_marble = iter.skip(5).next().unwrap().parse().unwrap();

    Box::new((players, last_marble))
}

struct RingCursor<'a, T: 'a> {
    cursor: Cursor<'a, T>,
}

impl<'a, T> RingCursor<'a, T> {
    fn insert(&mut self, elem: T) {
        self.cursor.insert(elem)
    }
    fn remove(&mut self) -> T {
        match self.cursor.remove() {
            Some(elem) => elem,
            None => self.cursor.remove().unwrap(),
        }
    }
    fn seek_forward(&mut self, n: usize) {
        for _ in 0..n {
            if self.cursor.next().is_none() {
                self.cursor.next().unwrap();
            }
        }
    }
    fn seek_backward(&mut self, n: usize) {
        for _ in 0..n {
            if self.cursor.prev().is_none() {
                self.cursor.prev().unwrap();
            }
        }
    }
}

#[aoc(day9, part1)]
pub fn part1((players, last_marble): &(usize, usize)) -> usize {
    let mut scores = vec![0; *players];
    let mut circle = LinkedList::new();
    let mut cursor = RingCursor { cursor: circle.cursor() };
    cursor.insert(0);

    for (marble, player) in (1..=*last_marble).zip((0..*players).cycle()) {
        if marble % 23 == 0 {
            scores[player] += marble;
            cursor.seek_backward(7);
            scores[player] += cursor.remove();
        } else {
            cursor.seek_forward(2);
            cursor.insert(marble);
        }
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("10 players; last marble is worth 1618 points"),
            Box::new((10, 1618))
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&(9, 25)), 32);
        assert_eq!(part1(&(10, 1618)), 8317);
        assert_eq!(part1(&(13, 7999)), 146373);
        assert_eq!(part1(&(17, 1104)), 2764);
        assert_eq!(part1(&(21, 6111)), 54718);
        assert_eq!(part1(&(30, 5807)), 37305);
    }
}
