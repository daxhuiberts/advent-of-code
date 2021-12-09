use aoctools::grid::Grid;
use aoctools::main;
use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;

main!("day09", parse_input);

fn parse_input(input: &str) -> Grid<u32> {
    parse_input_inner(input, 100, 100)
}

fn parse_input_inner(input: &str, width: usize, height: usize) -> Grid<u32> {
    let data = input
        .lines()
        .flat_map(|line| {
            line.split("")
                .filter(|&x| !x.is_empty())
                .map(|digit| digit.parse().unwrap())
        })
        .collect();
    Grid::new_with_data(width, height, data)
}

fn part1(grid: &Grid<u32>) -> u32 {
    low_points(grid).iter().map(|(_, cell)| cell + 1).sum()
}

fn part2(grid: &Grid<u32>) -> usize {
    low_points(grid)
        .into_iter()
        .map(|(cord, cell)| {
            let mut set: HashSet<(usize, usize)> = HashSet::new();
            let mut process_list = VecDeque::new();
            set.insert(cord);
            process_list.push_back((cord, cell));
            while let Some((cord, value)) = process_list.pop_front() {
                grid.adjacent_values(cord, false)
                    .for_each(|(new_cord, new_value)| {
                        if new_value > value && new_value < 9 && !set.contains(&new_cord) {
                            set.insert(new_cord);
                            process_list.push_back((new_cord, new_value));
                        }
                    });
            }
            set.len()
        })
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

fn low_points(grid: &Grid<u32>) -> Vec<((usize, usize), u32)> {
    grid.cell_with_cords()
        .filter(|&((x, y), &cell)| {
            grid.adjacent_values((x, y), false)
                .all(|(_, neighbour)| neighbour > cell)
        })
        .map(|(cord, value)| (cord, *value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    " };

    lazy_static! {
        static ref INPUT: Grid<u32> = Grid::new_with_data(
            10,
            5,
            vec![
                2, 1, 9, 9, 9, 4, 3, 2, 1, 0, 3, 9, 8, 7, 8, 9, 4, 9, 2, 1, 9, 8, 5, 6, 7, 8, 9, 8,
                9, 2, 8, 7, 6, 7, 8, 9, 6, 7, 8, 9, 9, 8, 9, 9, 9, 6, 5, 6, 7, 8,
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input_inner(INPUT_STR, 10, 5), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 1134);
    }
}
