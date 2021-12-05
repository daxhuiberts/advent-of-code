use aoctools::main;
use itertools::Itertools;

main!("day05", parse_input);

type Line = ((u32, u32), (u32, u32));

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .step_by(2)
                .map(|cord| {
                    cord.split(',')
                        .map(|number| number.parse().unwrap())
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn part1(lines: &[Line]) -> usize {
    let straight_lines = lines
        .iter()
        .filter(|line| is_straight_line(line))
        .copied()
        .collect_vec();
    count_overlaps(&straight_lines)
}

fn part2(lines: &[Line]) -> usize {
    count_overlaps(lines)
}

fn is_straight_line(&((lx, ly), (rx, ry)): &Line) -> bool {
    lx == rx || ly == ry
}

fn count_overlaps(lines: &[Line]) -> usize {
    let cells = lines_to_cells(lines);
    let counts = cells.iter().counts();
    let overlaps = counts.iter().filter(|(_, &count)| count > 1).count();
    overlaps
}

fn lines_to_cells(lines: &[Line]) -> Vec<(u32, u32)> {
    lines.iter().flat_map(|line| line_to_cells(line)).collect()
}

fn line_to_cells(&((lx, ly), (rx, ry)): &Line) -> Vec<(u32, u32)> {
    if lx == rx {
        range(ly, ry).into_iter().map(|y| (lx, y)).collect()
    } else if ly == ry {
        range(lx, rx).into_iter().map(|x| (x, ly)).collect()
    } else {
        range(lx, rx).into_iter().zip(range(ly, ry)).collect()
    }
}

fn range(a: u32, b: u32) -> Vec<u32> {
    if a <= b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    " };

    const INPUT: [Line; 10] = [
        ((0, 9), (5, 9)),
        ((8, 0), (0, 8)),
        ((9, 4), (3, 4)),
        ((2, 2), (2, 1)),
        ((7, 0), (7, 4)),
        ((6, 4), (2, 0)),
        ((0, 9), (2, 9)),
        ((3, 4), (1, 4)),
        ((0, 0), (8, 8)),
        ((5, 5), (8, 2)),
    ];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 12);
    }

    #[test]
    fn test_line_to_cells() {
        assert_eq!(line_to_cells(&((1, 1), (3, 1))), [(1, 1), (2, 1), (3, 1)]);
        assert_eq!(line_to_cells(&((3, 1), (1, 1))), [(3, 1), (2, 1), (1, 1)]);
        assert_eq!(line_to_cells(&((1, 1), (1, 3))), [(1, 1), (1, 2), (1, 3)]);
        assert_eq!(line_to_cells(&((1, 3), (1, 1))), [(1, 3), (1, 2), (1, 1)]);
        assert_eq!(line_to_cells(&((1, 1), (3, 3))), [(1, 1), (2, 2), (3, 3)]);
        assert_eq!(line_to_cells(&((3, 3), (1, 1))), [(3, 3), (2, 2), (1, 1)]);
    }

    #[test]
    fn test_range() {
        assert_eq!(range(1, 3), [1, 2, 3]);
        assert_eq!(range(3, 1), [3, 2, 1]);
    }
}
