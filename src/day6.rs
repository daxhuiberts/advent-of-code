use aoctools::IterExt;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            line.split(", ")
                .map(|coordinate| coordinate.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[(usize, usize)]) -> usize {
    let (x, y): (Vec<usize>, Vec<usize>) = input.iter().cloned().unzip();
    let (max_x, max_y) = (x.into_iter().max().unwrap(), y.into_iter().max().unwrap());

    let data: Vec<(usize, Option<usize>)> = (0..=max_y)
        .cartesian_product(0..=max_x)
        .map(|(y, x)| get_settler((x, y), input))
        .collect_vec();

    let edge_positions: HashSet<usize> = (0..=max_y)
        .cartesian_product(0..=max_x)
        .positions(|(y, x)| x == 0 || x == max_x || y == 0 || y == max_y)
        .collect();

    let exclude_indexes: HashSet<usize> = data
        .iter()
        .enumerate()
        .filter(|(position, _)| edge_positions.contains(position))
        .filter_map(|(_, (_score, some_index))| *some_index)
        .unique()
        .collect();

    let index_scores = data
        .iter()
        .filter_map(|(_score, some_index)| *some_index)
        .filter(|index| !exclude_indexes.contains(index))
        .group_count();

    *index_scores.values().max().unwrap()
}

#[aoc(day6, part2)]
pub fn part2(input: &[(usize, usize)]) -> usize {
    part2_inner(input, 10_000)
}

fn part2_inner(input: &[(usize, usize)], threshold: usize) -> usize {
    let (x, y): (Vec<usize>, Vec<usize>) = input.iter().cloned().unzip();
    let (max_x, max_y) = (x.into_iter().max().unwrap(), y.into_iter().max().unwrap());

    (0..=max_y)
        .cartesian_product(0..=max_x)
        .map(|(y, x)| {
            input
                .iter()
                .map(|(xx, yy)| manhattan_distance((x, y), (*xx, *yy)))
                .sum::<usize>()
        })
        .filter(|total_distance| *total_distance < threshold)
        .count()
}

fn get_settler(coordinate: (usize, usize), input: &[(usize, usize)]) -> (usize, Option<usize>) {
    input
        .iter()
        .enumerate()
        .fold((999, None), |(score, position), (index, (xx, yy))| {
            let new_score = manhattan_distance(coordinate, (*xx, *yy));
            match score.cmp(&new_score) {
                Ordering::Less => (score, position),
                Ordering::Equal => (score, None),
                Ordering::Greater => (new_score, Some(index)),
            }
        })
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (((a.0 as isize) - (b.0 as isize)).abs() + ((a.1 as isize) - (b.1 as isize)).abs()) as usize
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n";
    const PARSED: [(usize, usize); 6] = [(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT), PARSED);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&PARSED), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2_inner(&PARSED, 32), 16);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance((0, 0), (0, 0)), 0);
        assert_eq!(manhattan_distance((0, 0), (1, 1)), 2);
        assert_eq!(manhattan_distance((3, 3), (1, 5)), 4);
        assert_eq!(manhattan_distance((5, 4), (2, 3)), 4);
    }
}
