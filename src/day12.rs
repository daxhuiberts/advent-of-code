type Input = (Vec<bool>, Vec<(Vec<bool>, bool)>);

#[aoc_generator(day12)]
pub fn parse_input(input: &str) -> Input {
    fn state(char: &u8) -> bool {
        *char == b'#'
    }

    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let initial_state = first_line.as_bytes()[15..].iter().map(state).collect();

    let rules = lines
        .skip(1)
        .map(|line| {
            let check = line.as_bytes()[0..5].iter().map(state).collect();
            let result = state(&line.as_bytes()[9]);
            (check, result)
        })
        .collect();

    (initial_state, rules)
}

#[aoc(day12, part1)]
pub fn part1(input: &Input) -> i64 {
    run(input, 20)
}

#[aoc(day12, part2)]
pub fn part2(input: &Input) -> i64 {
    run(input, 50_000_000_000)
}

fn run((initial_state, rules): &Input, generations: i64) -> i64 {
    let mut state: Vec<bool> = initial_state.clone();
    let mut offset: i64 = 0;

    for generation in 1..=generations {
        for _ in 0..3 {
            if state[0..3] != [false, false, false] {
                state.insert(0, false);
                offset -= 1;
            }
            let end = state.len();
            if state[(end - 3)..end] != [false, false, false] {
                state.push(false);
            }
        }

        let new_state = state
            .windows(5)
            .map(|window| {
                rules
                    .iter()
                    .find(|(check, _)| check[..] == *window)
                    .map(|(_, result)| *result)
                    .unwrap_or(false)
            })
            .collect::<Vec<bool>>();

        if let Some(index) = (0..=2).find(|index| state[*index..].starts_with(&new_state)) {
            offset += (generations - generation + 1) * (index as i64);
            break;
        }

        state = new_state;
        offset += 2;
    }

    state
        .into_iter()
        .zip(offset..)
        .filter_map(|(plant, score)| if plant { Some(score) } else { None })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const TEST_INPUT_DATA: &'static str = indoc!(
        "
        initial state: #..#.#..##......###...###

        ...## => #
        ..#.. => #
        .#... => #
        .#.#. => #
        .#.## => #
        .##.. => #
        .#### => #
        #.#.# => #
        #.### => #
        ##.#. => #
        ##.## => #
        ###.. => #
        ###.# => #
        ####. => #
        ..... => .
    "
    );

    lazy_static! {
        static ref TEST_INPUT_RESULT: Input = {
            (
                vec![
                    true, false, false, true, false, true, false, false, true, true, false, false,
                    false, false, false, false, true, true, true, false, false, false, true, true,
                    true,
                ],
                vec![
                    (vec![false, false, false, true, true], true),
                    (vec![false, false, true, false, false], true),
                    (vec![false, true, false, false, false], true),
                    (vec![false, true, false, true, false], true),
                    (vec![false, true, false, true, true], true),
                    (vec![false, true, true, false, false], true),
                    (vec![false, true, true, true, true], true),
                    (vec![true, false, true, false, true], true),
                    (vec![true, false, true, true, true], true),
                    (vec![true, true, false, true, false], true),
                    (vec![true, true, false, true, true], true),
                    (vec![true, true, true, false, false], true),
                    (vec![true, true, true, false, true], true),
                    (vec![true, true, true, true, false], true),
                    (vec![false, false, false, false, false], false),
                ],
            )
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT_DATA), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), 325);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*TEST_INPUT_RESULT), 999999999374);
    }
}
