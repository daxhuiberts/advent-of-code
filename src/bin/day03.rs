use itertools::Itertools;
use std::collections::HashSet;
use WireDirection::*;

static INPUT: &str = include_str!("../../input/day03.txt");

fn main() {
    let (wire1, wire2): (&str, &str) = INPUT.lines().collect_tuple().unwrap();
    let (wire1, wire2) = (parse_wire_points(wire1), parse_wire_points(wire2));

    let closest_distance = closest_crossing_distance(&wire1, &wire2);
    println!("part 1: {}", closest_distance);
}

fn parse_wire_points(input: &str) -> Vec<WirePoint> {
    input.split(",").map(|point| {
        let direction = WireDirection::parse(&point[0..1]);
        let length = point[1..].parse().unwrap();
        WirePoint { direction, length }
    }).collect()
}

#[derive(PartialEq, Eq, Debug)]
enum WireDirection {
    Up,
    Right,
    Down,
    Left,
}

impl WireDirection {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Up,
            "R" => Right,
            "D" => Down,
            "L" => Left,
            _ => panic!("not matched"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct WirePoint {
    direction: WireDirection,
    length: u32,
}

fn calculate_wire_positions(wire: &[WirePoint]) -> HashSet<(i32, i32)> {
    let mut positions = HashSet::new();
    let mut current_position = (0, 0);

    for point in wire {
        let (x, y) = match point.direction {
            Up => (0, 1),
            Right => (1, 0),
            Down => (0, -1),
            Left => (-1, 0),
        };

        for _ in 0..point.length {
            current_position = (current_position.0 + x, current_position.1 + y);
            positions.insert(current_position);
        }
    };

    positions
}

fn closest_crossing_distance(wire1: &[WirePoint], wire2: &[WirePoint]) -> i32 {
    let wire1_positions = calculate_wire_positions(wire1);
    let wire2_positions = calculate_wire_positions(wire2);

    let crossings: HashSet<(i32, i32)> = wire1_positions.intersection(&wire2_positions).cloned().collect();
    let mut distances: Vec<i32> = crossings.into_iter().map(|crossing| crossing.0.abs() + crossing.1.abs()).collect();
    distances.sort();
    *distances.first().unwrap()
}

#[test]
fn test_parse_wire_points() {
    let wire_points = parse_wire_points("R8,U5,L5,D3");
    let expected = vec![
        WirePoint { direction: Right, length: 8 },
        WirePoint { direction: Up, length: 5 },
        WirePoint { direction: Left, length: 5 },
        WirePoint { direction: Down, length: 3 },
    ];

    assert_eq!(wire_points, expected);
}

#[test]
fn test_calculate_wire_positions() {
    let wire_points = parse_wire_points("R8,U5,L5,D3");
    let result = calculate_wire_positions(&wire_points);
    let expected = vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (7, 5), (6, 5), (5, 5), (4, 5), (3, 5), (3, 4), (3, 3), (3, 2)].into_iter().collect();
    assert_eq!(result, expected);
}

#[test]
fn test_closest_crossing_distance() {
    let wire1 = parse_wire_points("R8,U5,L5,D3");
    let wire2 = parse_wire_points("U7,R6,D4,L4");
    assert_eq!(closest_crossing_distance(&wire1, &wire2), 6);
}
