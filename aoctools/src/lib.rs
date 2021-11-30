mod iterext;

pub use iterext::IterExt;

#[macro_export]
macro_rules! main {
    ( $day:literal ) => {
        static INPUT: &str = include_str!(concat!("../../input/", $day, ".txt"));

        fn main() {
            println!("part 1: {}", part1(INPUT));
            println!("part 2: {}", part2(INPUT));
        }
    };
    ( $day:literal, parse_input ) => {
        static INPUT: &str = include_str!(concat!("../../input/", $day, ".txt"));

        fn main() {
            let input = parse_input(INPUT);
            println!("part 1: {}", part1(input.as_ref()));
            println!("part 2: {}", part2(input.as_ref()));
        }
    };
}

#[macro_export]
macro_rules! mapper {
    ( $( $l:literal => $r:literal ),* ) => {
        |input| {
            match input {
                $(
                    $l => $r,
                )*
                _ => panic!("failed to match"),
            }
        }
    };
}

#[macro_export]
macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            use std::collections::HashSet;
            let mut temp_set = HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}
