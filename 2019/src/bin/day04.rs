use itertools::Itertools;

static INPUT: &str = include_str!("../../input/day04.txt");

fn main() {
    let (lower, upper) = INPUT.trim().split("-").map(|value| value.parse().unwrap()).collect_tuple().unwrap();

    let passwords = count_passwords(lower, upper, check_password_simple);
    println!("part 1: {}", passwords);

    let passwords = count_passwords(lower, upper, check_password_advanced);
    println!("part 2: {}", passwords);
}

fn count_passwords(lower: u32, upper: u32, password_checker: fn(u32) -> bool) -> usize {
    (lower..upper).filter(|password| password_checker(*password)).count()
}

fn check_password_simple(password: u32) -> bool {
    let digits = number_to_digits(password);

    let mut sequence = false;

    for (left, right) in digits.iter().tuple_windows() {
        if left > right {
            return false;
        }
        if left == right {
            sequence = true;
        }
    }

    sequence
}

fn check_password_advanced(password: u32) -> bool {
    let digits = number_to_digits(password);

    let mut sequence_digits = vec![];

    for (left, right) in digits.iter().tuple_windows() {
        if left > right {
            return false;
        }
        if left == right {
            sequence_digits.push(left);
        }
    }

    sequence_digits.into_iter().any(|digit| digits.iter().filter(|d| *d == digit).count() == 2)
}

fn number_to_digits(mut number: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    while number > 9 {
        digits.push(number % 10);
        number = number / 10;
    }
    digits.push(number);
    digits.reverse();
    digits
}

#[test]
fn test_check_password_simple() {
    assert_eq!(check_password_simple(111111), true);
    assert_eq!(check_password_simple(123345), true);
    assert_eq!(check_password_simple(223450), false);
    assert_eq!(check_password_simple(123789), false);
}

#[test]
fn test_check_password_advanced() {
    assert_eq!(check_password_advanced(112233), true);
    assert_eq!(check_password_advanced(123444), false);
    assert_eq!(check_password_advanced(111122), true);
}

#[test]
fn test_number_to_digits() {
    assert_eq!(number_to_digits(123456), vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(number_to_digits(347312), vec![3, 4, 7, 3, 1, 2]);
    assert_eq!(number_to_digits(805915), vec![8, 0, 5, 9, 1, 5]);
}
