type Input = (Vec<bool>, Vec<(Vec<bool>, bool)>);

const GENERATIONS: usize = 20;

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
pub fn part1((initial_state, rules): &Input) -> i32 {
    let mut state: Vec<bool> = std::iter::repeat(false)
        .take(4 * GENERATIONS)
        .chain(initial_state.iter().cloned())
        .chain(std::iter::repeat(false).take(4 * GENERATIONS))
        .collect();

    print_state(&state, 0);

    for generation in 1..=GENERATIONS {
        state = state.windows(5).map(|window|
            rules.iter().find(|(check, _)| check[..] == *window).map(|(_, result)| *result).unwrap_or(false)
        ).collect();

        state.insert(0, false);
        state.insert(0, false);
        state.push(false);
        state.push(false);

        print_state(&state, generation);
    }

    state.into_iter().zip((-4 * GENERATIONS as i32)..).filter_map(|(plant, score)| if plant { Some(score) } else { None }).sum()
}

fn print_state(state: &[bool], generation: usize) {
    print!("{:2}: ", generation);
    for item in state {
        print!("{}", if *item { "#" } else { "." });
    }
    println!();
}

// #[aoc(day12, part2)]
// pub fn part2(input: &[i32]) -> i32 {
//     let iterator = std::iter::once(&0).chain(input.iter().cycle());
//     let cumulated = iterator.scan(0, |state, value| {
//         *state += value;
//         Some(*state)
//     });
//     let duplicates = cumulated.scan(HashSet::new(), |set, value| Some(set.replace(value)));
//     duplicates.filter_map(|value| value).next().unwrap()
// }

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

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&[1, -1]), 0);
    //     assert_eq!(part2(&[3, 3, 4, -2, -4]), 10);
    //     assert_eq!(part2(&[-6, 3, 8, 5, -6]), 5);
    //     assert_eq!(part2(&[7, 7, -2, -7, -4]), 14);
    // }
}
