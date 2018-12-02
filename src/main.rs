fn main() {
    let input = include_str!("../input.txt");

    let frequency = input.lines().fold(0, |acc, line| acc + line.parse::<i32>().unwrap());
    println!("{}", frequency);
}
