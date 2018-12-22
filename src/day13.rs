type Track = Vec<Vec<char>>;
type Cart = (usize, usize, char, usize);

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (Track, Vec<Cart>) {
    let mut track: Track = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut carts: Vec<Cart> = vec![];

    for (y, row) in track.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            match *cell {
                '^' => {
                    carts.push((x, y, '^', 0));
                    *cell = '|';
                },
                'v' => {
                    carts.push((x, y, 'v', 0));
                    *cell = '|';
                },
                '<' => {
                    carts.push((x, y, '<', 0));
                    *cell = '-';
                },
                '>' => {
                    carts.push((x, y, '>', 0));
                    *cell = '-';
                },
                _ => ()
            }
        }
    }

    (track, carts)
}

#[aoc(day13, part1)]
pub fn part1(input: &(Track, Vec<Cart>)) -> String {
    let (track, mut carts): (Track, Vec<Cart>) = input.clone();

    // print(&track, &carts);

    loop {
        let carts_copy = carts.clone();

        for (x, y, direction, step) in &mut carts {
            match *direction {
                '^' => *y -= 1,
                'v' => *y += 1,
                '<' => *x -= 1,
                '>' => *x += 1,
                _ => panic!("invalid direction"),
            }

            match (track[*y][*x], *direction) {
                ('/', '^') => *direction = '>',
                ('/', 'v') => *direction = '<',
                ('/', '<') => *direction = 'v',
                ('/', '>') => *direction = '^',
                ('\\', '^') => *direction = '<',
                ('\\', 'v') => *direction = '>',
                ('\\', '<') => *direction = '^',
                ('\\', '>') => *direction = 'v',
                ('+', _) => {
                    match (*direction, *step) {
                        ('^', 0) => *direction = '<',
                        ('^', 2) => *direction = '>',
                        ('v', 0) => *direction = '>',
                        ('v', 2) => *direction = '<',
                        ('<', 0) => *direction = 'v',
                        ('<', 2) => *direction = '^',
                        ('>', 0) => *direction = '^',
                        ('>', 2) => *direction = 'v',
                        _ => (),
                    };
                    *step = (*step + 1) % 3;
                }
                _ => (),
            }

            if carts_copy.iter().any(|(xx, yy, _, _)| x == xx && y == yy) {
                return format!("{},{}", x, y);
            }
        }

        for (x, y, _, _) in &carts {
            if carts.iter().filter(|(xx, yy, _, _)| x == xx && y == yy).count() > 1 {
                return format!("{},{}", x, y);
            }
        }
    }
}

#[allow(dead_code)]
fn print(track: &Track, carts: &Vec<Cart>) {
    for (y, row) in track.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let some_cart = carts.iter().filter(|(xx, yy, _, _)| x == *xx && y == *yy).collect::<Vec<&Cart>>();

            match some_cart[..] {
                [_, _] => print!("X"),
                [(_, _, direction, _)] => print!("{}", direction),
                [] => print!("{}", cell),
                _ => panic!("not allowed"),
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const TEST_INPUT_DATA: &'static str = indoc!(
        r"
        /->-\
        |   |  /----\
        | /-+--+-\  |
        | | |  | v  |
        \-+-/  \-+--/
          \------/
    "
    );

    const TEST_INPUT_TRACK_STR: &'static str = indoc!(
        r"
        /---\
        |   |  /----\
        | /-+--+-\  |
        | | |  | |  |
        \-+-/  \-+--/
          \------/
    "
    );

    lazy_static! {
        static ref TEST_INPUT_TRACK: Track = {
            TEST_INPUT_TRACK_STR.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
        };

        static ref TEST_INPUT_RESULT: (Track, Vec<Cart>) = {
            (TEST_INPUT_TRACK.clone(), vec![(2, 0, '>', 0), (9, 3, 'v', 0)])
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT_DATA), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), "7,3");
    }
}
