const GRID_WIDTH: i32 = 300;
const GRID_HEIGHT: i32 = 300;

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    let serial = input.parse().unwrap();
    let grid = generate_grid(serial);
    let square_values = generate_square_values(&grid, 3);

    let position = square_values.iter().enumerate().max_by_key(|(_, total)| *total).unwrap().0;

    format!("{},{}", (position as i32 % (GRID_WIDTH - 2)) + 1, (position as i32 / (GRID_WIDTH - 2)) + 1)
}

fn generate_square_values(grid: &[i32], size: i32) -> Vec<i32> {
    fn get(grid: &[i32], x: i32, y: i32) -> i32 {
        grid[((y - 1) * GRID_WIDTH + (x - 1)) as usize]
    }

    let mut square_values = vec![];

    for y in 1..=(GRID_HEIGHT - size + 1) {
        for x in 1..=(GRID_WIDTH - size + 1) {

            let value = get(grid, x, y) + get(grid, x + 1, y) + get(grid, x + 2, y) +
                get(grid, x, y + 1) + get(grid, x + 1, y + 1) + get(grid, x + 2, y + 1) +
                get(grid, x, y + 2) + get(grid, x + 1, y + 2) + get(grid, x + 2, y + 2);

            square_values.push(value);
        }
    }

    square_values
}

fn generate_grid(serial: i32) -> Vec<i32> {
    let mut grid = vec![];

    for y in 1..=GRID_HEIGHT {
        for x in 1..=GRID_WIDTH {
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

    #[test]
    fn test_calculate_power_level() {
        assert_eq!(calculate_power_level(8, 3, 5), 4);
        assert_eq!(calculate_power_level(57, 122, 79), -5);
        assert_eq!(calculate_power_level(39, 217, 196), 0);
        assert_eq!(calculate_power_level(71, 101, 153), 4);
    }
}
