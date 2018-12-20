const GRID_SIZE: i32 = 300;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let serial = input.parse().unwrap();
    let grid = generate_grid(serial);
    let square_values = generate_square_values(&grid, 3);
    let ((x, y), _value) = square_values
        .into_iter()
        .max_by_key(|(_, value)| *value)
        .unwrap();

    format!("{},{}", x, y)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    let serial = input.parse().unwrap();
    let grid = generate_grid(serial);

    let ((x, y), size, _) = (1..=20)
        .map(|size| {
            let square_values = generate_square_values(&grid, size);
            let (position, value) = square_values
                .into_iter()
                .max_by_key(|(_, value)| *value)
                .unwrap();
            (position, size, value)
        })
        .max_by_key(|(_, _, value)| *value)
        .unwrap();

    format!("{},{},{}", x, y, size)
}

fn generate_square_values(grid: &[i32], size: i32) -> Vec<((i32, i32), i32)> {
    let mut square_values = vec![];

    for y_base in 1..=(GRID_SIZE - size + 1) {
        for x_base in 1..=(GRID_SIZE - size + 1) {
            let mut value = 0;

            for y_offset in 0..size {
                for x_offset in 0..size {
                    value += grid
                        [((y_base + y_offset - 1) * GRID_SIZE + (x_base + x_offset - 1)) as usize];
                }
            }

            square_values.push(((x_base, y_base), value));
        }
    }

    square_values
}

fn generate_grid(serial: i32) -> Vec<i32> {
    let mut grid = vec![];

    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            grid.push(calculate_power_level(serial, x, y));
        }
    }

    grid
}

fn calculate_power_level(serial: i32, x: i32, y: i32) -> i32 {
    (((((((x + 10) * y) + serial) * (x + 10)) % 1000) / 100) - 5)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("18"), "33,45");
        assert_eq!(part1("42"), "21,61");
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2("18"), "90,269,16");
    //     assert_eq!(part2("42"), "232,251,12");
    // }

    #[test]
    fn test_calculate_power_level() {
        assert_eq!(calculate_power_level(8, 3, 5), 4);
        assert_eq!(calculate_power_level(57, 122, 79), -5);
        assert_eq!(calculate_power_level(39, 217, 196), 0);
        assert_eq!(calculate_power_level(71, 101, 153), 4);
    }
}
