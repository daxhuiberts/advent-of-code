type Track = Vec<Vec<char>>;
type Cart = (usize, usize, usize, char, usize);

#[aoc_generator(day13)]
pub fn parse_input(input: &str) -> (Track, Vec<Cart>) {
    let mut track: Track = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut carts: Vec<Cart> = vec![];

    for (y, row) in track.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            if ['^', 'v', '<', '>'].contains(cell) {
                carts.push((carts.len() + 1, x, y, *cell, 0));
            }
            *cell = match *cell {
                '^' => '|',
                'v' => '|',
                '<' => '-',
                '>' => '-',
                x => x,
            }
        }
    }

    (track, carts)
}

#[aoc(day13, part1)]
pub fn part1(input: &(Track, Vec<Cart>)) -> String {
    run(input, true)
}

#[aoc(day13, part2)]
pub fn part2(input: &(Track, Vec<Cart>)) -> String {
    run(input, false)
}

fn run(input: &(Track, Vec<Cart>), first_crash: bool) -> String {
    let (track, mut carts): (Track, Vec<Cart>) = input.clone();

    // print(&track, &carts);

    for generation in 1.. {

        let mut crashed: Vec<usize> = vec![];

        carts.sort_by_key(|(_, x, y, _, _)| (*y, *x));

        for index in 0..carts.len() {
            {
                let (_, x, y, direction, step) = carts.get_mut(index).unwrap();

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
            }

            let (i, x, y, _, _) = carts.get(index).unwrap();

            if let Some((ii, _, _, _, _)) = carts
                .iter()
                .find(|(ii, xx, yy, _, _)| i != ii && x == xx && y == yy)
            {
                crashed.push(*i);
                crashed.push(*ii);
            }
        }

        if !crashed.is_empty() {
            if first_crash {
                let cart = carts
                    .iter()
                    .find(|(index, _, _, _, _)| crashed[1] == *index)
                    .unwrap();
                return format!("{},{} (generation {})", cart.1, cart.2, generation);
            } else {
                carts.retain(|(index, _, _, _, _)| !crashed.contains(&index));
                if carts.len() == 1 {
                    return format!(
                        "{} -> {},{} (generation {})",
                        carts[0].0, carts[0].1, carts[0].2, generation
                    );
                }
            }
        }
    }

    String::new()
}

#[allow(dead_code)]
fn print(track: &Track, carts: &Vec<Cart>) {
    for (y, row) in track.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let some_cart = carts
                .iter()
                .filter(|(_, xx, yy, _, _)| x == *xx && y == *yy)
                .collect::<Vec<&Cart>>();

            match some_cart[..] {
                [_, _] => print!("X"),
                [(_, _, _, direction, _)] => print!("{}", direction),
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
            TEST_INPUT_TRACK_STR
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect()
        };
        static ref TEST_INPUT_RESULT: (Track, Vec<Cart>) = {
            (
                TEST_INPUT_TRACK.clone(),
                vec![(0, 2, 0, '>', 0), (1, 9, 3, 'v', 0)],
            )
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
