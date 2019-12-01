static INPUT: &str = include_str!("day01.txt");

fn main() {
    let input: Vec<i32> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    let fuel = sum_modules(&input, calculate_fuel);
    println!("part 1: {}", fuel);
    let total_fuel = sum_modules(&input, calculate_total_fuel);
    println!("part 2: {}", total_fuel);
}

fn sum_modules(modules: &[i32], calculator: fn(i32) -> i32) -> i32 {
    modules.iter().map(|module| calculator(*module)).sum()
}

fn calculate_total_fuel(weight: i32) -> i32 {
    let mut total_fuel = 0;
    let mut next_fuel = calculate_fuel(weight);

    while next_fuel > 0 {
        total_fuel += next_fuel;
        next_fuel = calculate_fuel(next_fuel);
    }

    total_fuel
}

fn calculate_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

#[test]
fn test_sum_modules() {
    assert_eq!(sum_modules(&vec![12, 14, 1969, 100756], calculate_fuel), 34241);
}

#[test]
fn test_calculate_total_fuel() {
    assert_eq!(calculate_total_fuel(14), 2);
    assert_eq!(calculate_total_fuel(1969), 966);
    assert_eq!(calculate_total_fuel(100756), 50346);
}

#[test]
fn test_calculate_fuel() {
    assert_eq!(calculate_fuel(12), 2);
    assert_eq!(calculate_fuel(14), 2);
    assert_eq!(calculate_fuel(1969), 654);
    assert_eq!(calculate_fuel(100756), 33583);
}
