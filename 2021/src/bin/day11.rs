use aoctools::grid::Grid;
use aoctools::main;

main!("day11", parse_input);

fn parse_input(input: &str) -> Grid<u8> {
    parse_input_inner(input, 10, 10)
}

fn parse_input_inner(input: &str, width: usize, height: usize) -> Grid<u8> {
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

fn part1(grid: &Grid<u8>) -> usize {
    (0..100)
        .scan(grid.clone(), |gen, _| {
            *gen = generation(&gen);
            Some(gen.data().iter().filter(|cell| **cell == 0).count())
        })
        .sum()
}

fn part2(grid: &Grid<u8>) -> usize {
    (0..)
        .scan(grid.clone(), |gen, _| {
            *gen = generation(&gen);
            Some(gen.data().iter().filter(|cell| **cell == 0).count())
        })
        .position(|explosions| explosions == 100)
        .unwrap()
        + 1
}

fn generation(grid: &Grid<u8>) -> Grid<u8> {
    let mut grid = grid.clone();
    let mut queue = vec![];

    queue.extend(grid.cell_with_cords().map(|(cord, _)| cord));

    while let Some(cord) = queue.pop() {
        let cell = grid.get_mut(cord);
        *cell += 1;
        if *cell == 10 {
            // explode
            for (new_cord, _) in grid.adjacent_values(cord, true) {
                queue.push(new_cord);
            }
        }
    }

    for (_, cell) in grid.cell_with_cords_mut() {
        if *cell > 9 {
            *cell = 0;
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    fn g(input: &str, width: usize, height: usize) -> Grid<u8> {
        parse_input_inner(input, width, height)
    }

    const INPUT_STR: &'static str = indoc! { "
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    " };

    lazy_static! {
        static ref INPUT: Grid<u8> = Grid::new_with_data(
            10,
            10,
            vec![
                5, 4, 8, 3, 1, 4, 3, 2, 2, 3, 2, 7, 4, 5, 8, 5, 4, 7, 1, 1, 5, 2, 6, 4, 5, 5, 6, 1,
                7, 3, 6, 1, 4, 1, 3, 3, 6, 1, 4, 6, 6, 3, 5, 7, 3, 8, 5, 4, 7, 8, 4, 1, 6, 7, 5, 2,
                4, 6, 4, 5, 2, 1, 7, 6, 8, 4, 1, 7, 2, 1, 6, 8, 8, 2, 8, 8, 1, 1, 3, 4, 4, 8, 4, 6,
                8, 4, 8, 5, 5, 4, 5, 2, 8, 3, 7, 5, 1, 5, 2, 6
            ]
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 1656);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 195);
    }

    #[test]
    fn test_generation() {
        let generation0 = g(
            indoc! { "
            11111
            19991
            19191
            19991
            11111
        " },
            5,
            5,
        );
        let generation1 = g(
            indoc! { "
            34543
            40004
            50005
            40004
            34543
        " },
            5,
            5,
        );
        let generation2 = g(
            indoc! { "
            45654
            51115
            61116
            51115
            45654
        " },
            5,
            5,
        );

        assert_eq!(generation(&generation0), generation1);
        assert_eq!(generation(&generation1), generation2);

        let generation100 = g(
            indoc! { "
                0397666866
                0749766918
                0053976933
                0004297822
                0004229892
                0053222877
                0532222966
                9322228966
                7922286866
                6789998766
        " },
            10,
            10,
        );

        let result = (0..100).fold(INPUT.clone(), |gen, _| generation(&gen));

        assert_eq!(result, generation100);
    }
}
