use itertools::Itertools;
use std::collections::HashSet;
use std::collections::HashMap;
use WireDirection::*;

static INPUT: &str = include_str!("../../input/day03.txt");

fn main() {
    let (wire1, wire2): (&str, &str) = INPUT.lines().collect_tuple().unwrap();
    let (wire1, wire2) = (parse_wire_points(wire1), parse_wire_points(wire2));

    let closest_distance = closest_crossing_distance(&wire1, &wire2);
    println!("part 1: {}", closest_distance);

    let shortest_length = shortest_crossing_length(&wire1, &wire2);
    println!("part 2: {}", shortest_length);
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

fn calculate_wire_positions(wire: &[WirePoint]) -> HashMap<(i32, i32), i32> {
    let mut positions = HashMap::new();
    let mut current_position = (0, 0);
    let mut current_length = 0;

    for point in wire {
        let (x, y) = match point.direction {
            Up => (0, 1),
            Right => (1, 0),
            Down => (0, -1),
            Left => (-1, 0),
        };

        for _ in 0..point.length {
            current_position = (current_position.0 + x, current_position.1 + y);
            current_length += 1;
            positions.entry(current_position).or_insert(current_length);
        }
    };

    positions
}

fn closest_crossing_distance(wire1: &[WirePoint], wire2: &[WirePoint]) -> i32 {
    let wire1_positions: HashSet<(i32, i32)> = calculate_wire_positions(wire1).keys().cloned().collect();
    let wire2_positions: HashSet<(i32, i32)> = calculate_wire_positions(wire2).keys().cloned().collect();

    let crossings: HashSet<(i32, i32)> = wire1_positions.intersection(&wire2_positions).cloned().collect();
    let mut distances: Vec<i32> = crossings.into_iter().map(|crossing| crossing.0.abs() + crossing.1.abs()).collect();
    distances.sort();
    *distances.first().unwrap()
}

fn shortest_crossing_length(wire1: &[WirePoint], wire2: &[WirePoint]) -> i32 {
    let wire1_positions_with_length = calculate_wire_positions(wire1);
    let wire2_positions_with_length = calculate_wire_positions(wire2);

    let wire1_positions: HashSet<(i32, i32)> = wire1_positions_with_length.keys().cloned().collect();
    let wire2_positions: HashSet<(i32, i32)> = wire2_positions_with_length.keys().cloned().collect();

    let crossings: HashSet<(i32, i32)> = wire1_positions.intersection(&wire2_positions).cloned().collect();

    let mut lengths: Vec<i32> = crossings.iter().map(|position| {
        wire1_positions_with_length.get(position).unwrap() + wire2_positions_with_length.get(position).unwrap()
    }).collect();
    lengths.sort();
    *lengths.first().unwrap()
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
    let expected = vec![((1, 0), 1), ((2, 0), 2), ((3, 0), 3), ((4, 0), 4), ((5, 0), 5), ((6, 0), 6), ((7, 0), 7), ((8, 0), 8), ((8, 1), 9), ((8, 2), 10), ((8, 3), 11), ((8, 4), 12), ((8, 5), 13), ((7, 5), 14), ((6, 5), 15), ((5, 5), 16), ((4, 5), 17), ((3, 5), 18), ((3, 4), 19), ((3, 3), 20), ((3, 2), 21)].into_iter().collect();
    assert_eq!(result, expected);
}

#[test]
fn test_closest_crossing_distance() {
    let wire1 = parse_wire_points("R8,U5,L5,D3");
    let wire2 = parse_wire_points("U7,R6,D4,L4");
    assert_eq!(closest_crossing_distance(&wire1, &wire2), 6);
}

#[test]
fn test_shortest_crossing_length() {
    let wire1 = parse_wire_points("R8,U5,L5,D3");
    let wire2 = parse_wire_points("U7,R6,D4,L4");
    assert_eq!(shortest_crossing_length(&wire1, &wire2), 30);

    let wire1 = parse_wire_points("R75,D30,R83,U83,L12,D49,R71,U7,L72");
    let wire2 = parse_wire_points("U62,R66,U55,R34,D71,R55,D58,R83");
    assert_eq!(shortest_crossing_length(&wire1, &wire2), 610);

    let wire1 = parse_wire_points("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
    let wire2 = parse_wire_points("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
    assert_eq!(shortest_crossing_length(&wire1, &wire2), 410);
}
