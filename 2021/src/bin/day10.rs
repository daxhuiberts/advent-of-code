use aoctools::main;
use aoctools::mapper;
use itertools::Itertools;

main!("day10");

fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|result| match process_line(result) {
            Err(Error::Corrupt(bracket)) => Some(
                mapper!(')' => 3, ']' => 57, '}' => 1197, '>' => 25137)(bracket),
            ),
            _ => None,
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let results = input
        .lines()
        .filter_map(|result| match process_line(result) {
            Err(Error::Incomplete(brackets)) => Some(
                brackets
                    .into_iter()
                    .rev()
                    .map(mapper!('(' => 1, '[' => 2, '{' => 3, '<' => 4))
                    .fold(0, |score, value| score * 5 + value),
            ),
            _ => None,
        })
        .sorted()
        .collect_vec();
    results[results.len() / 2]
}

enum Error {
    Corrupt(char),
    Incomplete(Vec<char>),
}

fn process_line(line: &str) -> Result<(), Error> {
    let mut stack = vec![];
    for char in line.chars() {
        // println!("stack: {} char: {}", stack.iter().collect::<String>(), char);
        match char {
            '(' => stack.push('('),
            '[' => stack.push('['),
            '{' => stack.push('{'),
            '<' => stack.push('<'),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return Err(Error::Corrupt(')'));
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return Err(Error::Corrupt(']'));
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return Err(Error::Corrupt('}'));
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return Err(Error::Corrupt('>'));
                }
            }
            _ => panic!("char {} not allowed", char),
        }
    }

    if stack.is_empty() {
        Ok(())
    } else {
        Err(Error::Incomplete(stack))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &'static str = indoc! { "
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    " };

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 26397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 288957);
    }
}
