use aoctools::main;
use itertools::Itertools;

main!("day13", parse_input);

type Input = (Vec<(usize, usize)>, Vec<(char, usize)>);

fn parse_input(input: &str) -> Input {
    let (dots, folds) = input.split("\n\n").collect_tuple().unwrap();
    let dots = dots
        .lines()
        .map(|line| {
            line.split(",")
                .map(|number| number.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    let folds = folds
        .lines()
        .map(|line| {
            let (axis, offset) = line.split("=").collect_tuple().unwrap();
            (axis.chars().last().unwrap(), offset.parse().unwrap())
        })
        .collect_vec();
    (dots, folds)
}

fn part1((dots, folds): &Input) -> usize {
    let mut result = folds
        .iter()
        .take(1)
        .fold(dots.clone(), |dots, (axis, offset)| {
            fold(&dots, *axis, *offset)
        });
    result.sort_unstable();
    result.dedup();
    result.len()
}

fn part2((dots, folds): &Input) -> usize {
    let mut result = folds.iter().fold(dots.clone(), |dots, (axis, offset)| {
        fold(&dots, *axis, *offset)
    });
    result.sort_unstable();
    result.dedup();
    for y in 0..=5 {
        for x in 0..=38 {
            if result.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    unimplemented!();
}

fn fold(dots: &[(usize, usize)], axis: char, offset: usize) -> Vec<(usize, usize)> {
    match axis {
        'x' => dots
            .iter()
            .map(|(x, y)| {
                (
                    if x > &offset {
                        offset - (x - offset)
                    } else {
                        *x
                    },
                    *y,
                )
            })
            .collect_vec(),
        'y' => dots
            .iter()
            .map(|(x, y)| {
                (
                    *x,
                    if y > &offset {
                        offset - (y - offset)
                    } else {
                        *y
                    },
                )
            })
            .collect_vec(),
        _ => panic!("not allowed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5
    " };

    lazy_static! {
        static ref INPUT: Input = (
            vec![
                (6, 10),
                (0, 14),
                (9, 10),
                (0, 3),
                (10, 4),
                (4, 11),
                (6, 0),
                (6, 12),
                (4, 1),
                (0, 13),
                (10, 12),
                (3, 4),
                (3, 0),
                (8, 4),
                (1, 10),
                (2, 14),
                (8, 10),
                (9, 0)
            ],
            vec![('y', 7), ('x', 5)]
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 17);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 0);
    }
}
