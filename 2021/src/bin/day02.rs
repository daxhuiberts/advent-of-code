use aoctools::main;
use itertools::Itertools;

main!("day02", parse_input);

#[derive(Debug, PartialEq, Eq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let (action, value) = line.split(' ').collect_tuple().unwrap();
            let value = value.parse().unwrap();
            match action {
                "forward" => Command::Forward(value),
                "down" => Command::Down(value),
                "up" => Command::Up(value),
                _ => panic!(),
            }
        })
        .collect()
}

fn part1(input: &[Command]) -> u32 {
    #[derive(Default)]
    struct Position {
        distance: u32,
        depth: u32,
    }

    let position = input
        .iter()
        .fold(Position::default(), |pos, command| match command {
            Command::Forward(value) => Position {
                distance: pos.distance + value,
                ..pos
            },
            Command::Down(value) => Position {
                depth: pos.depth + value,
                ..pos
            },
            Command::Up(value) => Position {
                depth: pos.depth - value,
                ..pos
            },
        });
    position.distance * position.depth
}

fn part2(input: &[Command]) -> u32 {
    #[derive(Default)]
    struct Position {
        distance: u32,
        depth: u32,
        aim: u32,
    }

    let position = input
        .iter()
        .fold(Position::default(), |pos, command| match command {
            Command::Forward(value) => Position {
                distance: pos.distance + value,
                depth: pos.depth + pos.aim * value,
                ..pos
            },
            Command::Down(value) => Position {
                aim: pos.aim + value,
                ..pos
            },
            Command::Up(value) => Position {
                aim: pos.aim - value,
                ..pos
            },
        });
    position.distance * position.depth
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT_STR: &'static str = indoc! { "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    " };

    const INPUT: [Command; 6] = [
        Command::Forward(5),
        Command::Down(5),
        Command::Forward(8),
        Command::Up(3),
        Command::Down(8),
        Command::Forward(2),
    ];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 150);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 900);
    }
}
