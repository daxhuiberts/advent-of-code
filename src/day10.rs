use regex::Regex;

#[aoc_generator(day10)]
pub fn parse_input(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let regex =
        Regex::new(r"\Aposition=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>\z")
            .unwrap();

    input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let values: Vec<_> = captures
                .iter()
                .skip(1)
                .map(|capture| capture.unwrap().as_str().parse().unwrap())
                .collect();
            ((values[0], values[1]), (values[2], values[3]))
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[((i32, i32), (i32, i32))]) -> String {
    let (mut positions, velocities): (Vec<(i32, i32)>, Vec<(i32, i32)>) =
        input.iter().cloned().unzip();

    let dimensions = get_dimensions(&positions);
    let mut size = get_size(dimensions);

    loop {
        let new_positions = positions
            .iter()
            .zip(velocities.iter())
            .map(|(position, velocity)| (position.0 + velocity.0, position.1 + velocity.1))
            .collect::<Vec<_>>();

        let new_dimensions = get_dimensions(&new_positions);
        let new_size = get_size(new_dimensions);

        if new_size > size {
            break;
        } else {
            positions = new_positions;
            size = new_size;
        }
    }

    let print_dimensions = get_dimensions(&positions);
    generate_grid(&positions, print_dimensions)
}

fn get_dimensions(positions: &[(i32, i32)]) -> ((i32, i32), (i32, i32)) {
    let min_x = positions.iter().map(|(x, _)| *x).min().unwrap();
    let min_y = positions.iter().map(|(_, y)| *y).min().unwrap();
    let max_x = positions.iter().map(|(x, _)| *x).max().unwrap();
    let max_y = positions.iter().map(|(_, y)| *y).max().unwrap();

    ((min_x, max_x), (min_y, max_y))
}

fn get_size(((min_x, max_x), (min_y, max_y)): ((i32, i32), (i32, i32))) -> i64 {
    (max_x as i64 - min_x as i64) * (max_y as i64 - min_y as i64)
}

fn generate_grid(
    positions: &[(i32, i32)],
    ((min_x, max_x), (min_y, max_y)): ((i32, i32), (i32, i32)),
) -> String {
    let mut output = String::new();
    output.push('\n');

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.iter().any(|(xx, yy)| x == *xx && y == *yy) {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const TEST_INPUT_DATA: &'static str = indoc!(
        "
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>
    "
    );

    lazy_static! {
        static ref TEST_INPUT_RESULT: Vec<((i32, i32), (i32, i32))> = {
            vec![
                ((9, 1), (0, 2)),
                ((7, 0), (-1, 0)),
                ((3, -2), (-1, 1)),
                ((6, 10), (-2, -1)),
                ((2, -4), (2, 2)),
                ((-6, 10), (2, -2)),
                ((1, 8), (1, -1)),
                ((1, 7), (1, 0)),
                ((-3, 11), (1, -2)),
                ((7, 6), (-1, -1)),
                ((-2, 3), (1, 0)),
                ((-4, 3), (2, 0)),
                ((10, -3), (-1, 1)),
                ((5, 11), (1, -2)),
                ((4, 7), (0, -1)),
                ((8, -2), (0, 1)),
                ((15, 0), (-2, 0)),
                ((1, 6), (1, 0)),
                ((8, 9), (0, -1)),
                ((3, 3), (-1, 1)),
                ((0, 5), (0, -1)),
                ((-2, 2), (2, 0)),
                ((5, -2), (1, 2)),
                ((1, 4), (2, 1)),
                ((-2, 7), (2, -2)),
                ((3, 6), (-1, -1)),
                ((5, 0), (1, 0)),
                ((-6, 0), (2, 0)),
                ((5, 9), (1, -2)),
                ((14, 7), (-2, 0)),
                ((-3, 6), (2, -1)),
            ]
        };
    }

    const OUTPUT: &'static str = indoc!(
        "

        #...#..###
        #...#...#.
        #...#...#.
        #####...#.
        #...#...#.
        #...#...#.
        #...#...#.
        #...#..###
    "
    );

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT_DATA), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), OUTPUT);
    }
}
