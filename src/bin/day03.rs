use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day03.txt");

fn main() {
    let (wire1, wire2): (&str, &str) = INPUT.lines().collect_tuple().unwrap();
    let (wire1, wire2) = (parse_wire_points(wire1), parse_wire_points(wire2));

    println!("wire1: {:?}; wire2: {:?}", wire1, wire2);
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
        use WireDirection::*;

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

#[test]
fn test_parse_wire_points() {
    use WireDirection::*;

    let wire_points = parse_wire_points("R8,U5,L5,D3");
    let expected = vec![
        WirePoint { direction: Right, length: 8 },
        WirePoint { direction: Up, length: 5 },
        WirePoint { direction: Left, length: 5 },
        WirePoint { direction: Down, length: 3 },
    ];

    assert_eq!(wire_points, expected);
}
