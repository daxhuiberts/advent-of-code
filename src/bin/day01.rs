use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day01.txt");
    let parsed: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let frequency = parsed
        .iter()
        .fold(0, |acc, value| acc + value);
    println!("{}", frequency);

    let duplicate = parsed
        .iter()
        .cycle()
        .scan(0, |state, value| { *state += value; Some(*state) })
        .scan(HashSet::new(), |set, value| Some(set.replace(value)))
        .filter_map(|value| value)
        .next()
        .unwrap();
    println!("{:?}", duplicate);
}
