#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Box<(usize, usize)> {
    let mut iter = input.split_whitespace();
    let players = iter.next().unwrap().parse().unwrap();
    let last_marble = iter.skip(5).next().unwrap().parse().unwrap();

    Box::new((players, last_marble))
}

#[aoc(day9, part1)]
pub fn part1((players, last_marble): &(usize, usize)) -> usize {
    let mut scores = vec![0; *players];

    let mut current_marble_index = 0;
    let mut circle = vec![0];

    for (marble, player) in (1..=*last_marble).zip((0..*players).cycle()) {
        let current_length = circle.len();

        if marble % 23 == 0 {
            scores[player] += marble;
            current_marble_index = (current_marble_index + current_length - 7) % current_length;
            scores[player] += circle.remove(current_marble_index);
        } else {
            current_marble_index = ((current_marble_index + 1) % current_length) + 1;
            circle.insert(current_marble_index, marble);
        }
    }

    scores.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("10 players; last marble is worth 1618 points"), Box::new((10, 1618)));
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
