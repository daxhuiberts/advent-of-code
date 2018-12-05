use std::collections::HashSet;
use itertools::Itertools;
use iterext::IterExt;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input.trim().chars().scan_ref((0, 0), update_direction).fold_ref(set![(0, 0)], |set, position| {
        set.insert(position);
    }).len()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let (santa, robot) = input.trim().chars().tee();
    let santa_iter = santa.step_by(2).scan_ref((0, 0), update_direction);
    let robot_iter = robot.skip(1).step_by(2).scan_ref((0, 0), update_direction);

    santa_iter.chain(robot_iter).fold_ref(set![(0, 0)], |set, position| {
        set.insert(position);
    }).len()
}

fn update_direction(position: &mut (i32, i32), direction: char) {
    match direction {
        '^' => position.0 += 1,
        '>' => position.1 += 1,
        'v' => position.0 -= 1,
        '<' => position.1 -= 1,
        _ => panic!("invalid direction"),
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(">"), 2);
    assert_eq!(part1("^>v<"), 4);
    assert_eq!(part1("^v^v^v^v^v"), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2("^v"), 3);
    assert_eq!(part2("^>v<"), 3);
    assert_eq!(part2("^v^v^v^v^v"), 11);
}
