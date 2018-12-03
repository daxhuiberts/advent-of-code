use regex::Regex;
use itertools::Itertools;
use super::IterExt;

#[derive(Debug, PartialEq, Eq)]
pub struct Claim {
    id: usize,
    xoffset: usize,
    yoffset: usize,
    width: usize,
    height: usize,
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Claim> {
    let regex = Regex::new(r"\A#(\d+) @ (\d+),(\d+): (\d+)x(\d+)\z").unwrap();

    input.lines().map(|line| {
        let captures = regex.captures(line).unwrap();
        let values: Vec<_> = captures.iter().skip(1).map(|capture|
            capture.unwrap().as_str().parse().unwrap()
        ).collect();

        Claim {
            id: values[0],
            xoffset: values[1],
            yoffset: values[2],
            width: values[3],
            height: values[4],
        }
    }).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Claim]) -> usize {
    let map = input.iter().flat_map(|claim| {
        let xrange = claim.xoffset..(claim.xoffset + claim.width);
        let yrange = claim.yoffset..(claim.yoffset + claim.height);
        xrange.cartesian_product(yrange)
    }).group_count();

    map.values().filter(|count| count > &&1).count()
}

#[test]
fn test_parse_input() {
    let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    let result = vec![
        Claim { id: 1, xoffset: 1, yoffset: 3, width: 4, height: 4 },
        Claim { id: 2, xoffset: 3, yoffset: 1, width: 4, height: 4 },
        Claim { id: 3, xoffset: 5, yoffset: 5, width: 2, height: 2 },
    ];
    assert_eq!(parse_input(input), result);
}

#[test]
fn test_part1() {
    let input = vec![
        Claim { id: 1, xoffset: 1, yoffset: 3, width: 4, height: 4 },
        Claim { id: 2, xoffset: 3, yoffset: 1, width: 4, height: 4 },
        Claim { id: 3, xoffset: 5, yoffset: 5, width: 2, height: 2 },
    ];
    assert_eq!(part1(&input), 4);
}
