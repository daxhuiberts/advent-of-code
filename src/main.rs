use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let frequency = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .fold(0, |acc, value| acc + value);
    println!("{}", frequency);

    let duplicate = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
        .scan(0, |state, value| { *state += value; Some(*state) })
        .scan(HashSet::new(), |set, value| Some(set.replace(value)))
        .find(|x|x.is_some())
        .unwrap()
        .unwrap();
    println!("{:?}", duplicate);
}
