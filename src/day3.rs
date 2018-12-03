use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Claim {
    id: usize,
    x: usize,
    y: usize,
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
            x: values[1],
            y: values[2],
            width: values[3],
            height: values[4],
        }
    }).collect()
}

#[test]
fn test_parse_input() {
    let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
    let result = vec![
        Claim { id: 1, x: 1, y: 3, width: 4, height: 4 },
        Claim { id: 2, x: 3, y: 1, width: 4, height: 4 },
        Claim { id: 3, x: 5, y: 5, width: 2, height: 2 },
    ];
    assert_eq!(parse_input(input), result);
}
