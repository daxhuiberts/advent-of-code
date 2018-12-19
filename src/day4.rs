use aoctools::IterExt;
use chrono::NaiveDate;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Guard(usize);

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Begin(Guard),
    Sleep,
    Awake,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    date: NaiveDate,
    hour: usize,
    minute: usize,
    action: Action,
}

fn parse_input_step1(input: &str) -> Vec<Record> {
    let regex_line = Regex::new(r"\A\[(\d{4}-\d{2}-\d{2}) (23|00):(\d\d)\] (.+)\z").unwrap();
    let regex_action_begin = Regex::new(r"\AGuard #(\d+) begins shift\z").unwrap();

    input
        .lines()
        .sorted()
        .map(|line| {
            let captures = regex_line.captures(line).unwrap();

            let date =
                NaiveDate::parse_from_str(captures.get(1).unwrap().as_str(), "%Y-%m-%d").unwrap();
            let hour = captures.get(2).unwrap().as_str().parse().unwrap();
            let minute = captures.get(3).unwrap().as_str().parse().unwrap();
            let action_str = captures.get(4).unwrap().as_str();

            let action = regex_action_begin
                .captures(action_str)
                .map(|captures| {
                    let guard_id = captures.get(1).unwrap().as_str().parse().unwrap();
                    Action::Begin(Guard(guard_id))
                })
                .unwrap_or_else(|| match action_str {
                    "falls asleep" => Action::Sleep,
                    "wakes up" => Action::Awake,
                    _ => panic!("invalid action str"),
                });

            let record = Record {
                date,
                hour,
                minute,
                action,
            };

            if (record.action == Action::Sleep || record.action == Action::Awake)
                && record.hour == 23
            {
                panic!("Can't fall asleep of wake up before midnight");
            }

            record
        })
        .collect_vec()
}

#[derive(Debug, PartialEq, Eq)]
struct Period {
    begin: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Shift {
    date: NaiveDate,
    guard_id: Guard,
    asleep: Vec<Period>,
}

fn parse_input_step2(input: &[Record]) -> Vec<Shift> {
    let mut result = vec![];
    let mut sleep_minute = 0;

    for record in input {
        match record.action {
            Action::Begin(guard_id) => {
                let date = if record.hour == 23 {
                    record.date.succ()
                } else {
                    record.date
                };
                result.push(Shift {
                    date,
                    guard_id,
                    asleep: vec![],
                });
            }
            Action::Sleep => sleep_minute = record.minute,
            Action::Awake => {
                let shift: &mut Shift = result.last_mut().unwrap();
                let sleep_period = Period {
                    begin: sleep_minute,
                    end: record.minute,
                };
                shift.asleep.push(sleep_period);
            }
        }
    }

    result
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Shift> {
    parse_input_step2(&parse_input_step1(input))
}

fn get_guards_with_sleepy_minutes(input: &[Shift]) -> HashMap<Guard, HashMap<usize, usize>> {
    input
        .iter()
        .grouped(|item| item.guard_id)
        .iter()
        .fold_to_map(|map, (guard, shifts)| {
            let sleepy_minutes = shifts
                .iter()
                .flat_map(|shifts| {
                    shifts
                        .asleep
                        .iter()
                        .flat_map(|period| period.begin..period.end)
                })
                .group_count();
            map.insert(*guard, sleepy_minutes);
        })
}

#[aoc(day4, part1)]
pub fn part1(input: &[Shift]) -> usize {
    let data = get_guards_with_sleepy_minutes(input);

    let sleepiest_guard = data
        .iter()
        .max_by_key(|(_guard_id, sleepy_minutes)| sleepy_minutes.values().sum::<usize>())
        .unwrap()
        .0;

    let sleepiest_minute = data[sleepiest_guard]
        .iter()
        .max_by_key(|(_minute, sleep_count)| *sleep_count)
        .unwrap()
        .0;

    sleepiest_guard.0 * *sleepiest_minute
}

#[aoc(day4, part2)]
pub fn part2(input: &[Shift]) -> usize {
    let data = get_guards_with_sleepy_minutes(input);

    let (guard, minute, _) = data
        .iter()
        .map(|(guard, sleepy_minutes)| {
            let (minute, sleep_count) = sleepy_minutes
                .iter()
                .max_by_key(|(_minute, sleep_count)| *sleep_count)
                .unwrap_or((&0, &0));
            (guard, minute, sleep_count)
        })
        .max_by_key(|(_guard, _minute, sleep_count)| *sleep_count)
        .unwrap();

    guard.0 * minute
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    // Use the test data from the description, but mess up chronological ordering.
    const TEST_INPUT_DATA: &'static str = indoc!(
        "
        [1518-11-03 00:05] Guard #10 begins shift
        [1518-11-01 00:00] Guard #10 begins shift
        [1518-11-01 23:58] Guard #99 begins shift
        [1518-11-05 00:03] Guard #99 begins shift
        [1518-11-04 00:02] Guard #99 begins shift
        [1518-11-04 00:46] wakes up
        [1518-11-03 00:29] wakes up
        [1518-11-01 00:55] wakes up
        [1518-11-02 00:50] wakes up
        [1518-11-05 00:55] wakes up
        [1518-11-01 00:25] wakes up
        [1518-11-02 00:40] falls asleep
        [1518-11-04 00:36] falls asleep
        [1518-11-03 00:24] falls asleep
        [1518-11-01 00:30] falls asleep
        [1518-11-01 00:05] falls asleep
        [1518-11-05 00:45] falls asleep
    "
    );

    lazy_static! {
        static ref TEST_INPUT_INTERMEDIATE_RESULT: Vec<Record> = {
            vec![
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    hour: 00,
                    minute: 00,
                    action: Action::Begin(Guard(10)),
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    hour: 00,
                    minute: 05,
                    action: Action::Sleep,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    hour: 00,
                    minute: 25,
                    action: Action::Awake,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    hour: 00,
                    minute: 30,
                    action: Action::Sleep,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    hour: 00,
                    minute: 55,
                    action: Action::Awake,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    hour: 23,
                    minute: 58,
                    action: Action::Begin(Guard(99)),
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 02),
                    hour: 00,
                    minute: 40,
                    action: Action::Sleep,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 02),
                    hour: 00,
                    minute: 50,
                    action: Action::Awake,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 03),
                    hour: 00,
                    minute: 05,
                    action: Action::Begin(Guard(10)),
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 03),
                    hour: 00,
                    minute: 24,
                    action: Action::Sleep,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 03),
                    hour: 00,
                    minute: 29,
                    action: Action::Awake,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 04),
                    hour: 00,
                    minute: 02,
                    action: Action::Begin(Guard(99)),
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 04),
                    hour: 00,
                    minute: 36,
                    action: Action::Sleep,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 04),
                    hour: 00,
                    minute: 46,
                    action: Action::Awake,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 05),
                    hour: 00,
                    minute: 03,
                    action: Action::Begin(Guard(99)),
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 05),
                    hour: 00,
                    minute: 45,
                    action: Action::Sleep,
                },
                Record {
                    date: NaiveDate::from_ymd(1518, 11, 05),
                    hour: 00,
                    minute: 55,
                    action: Action::Awake,
                },
            ]
        };
        static ref TEST_INPUT_RESULT: Vec<Shift> = {
            vec![
                Shift {
                    date: NaiveDate::from_ymd(1518, 11, 01),
                    guard_id: Guard(10),
                    asleep: vec![Period { begin: 05, end: 25 }, Period { begin: 30, end: 55 }],
                },
                Shift {
                    date: NaiveDate::from_ymd(1518, 11, 02),
                    guard_id: Guard(99),
                    asleep: vec![Period { begin: 40, end: 50 }],
                },
                Shift {
                    date: NaiveDate::from_ymd(1518, 11, 03),
                    guard_id: Guard(10),
                    asleep: vec![Period { begin: 24, end: 29 }],
                },
                Shift {
                    date: NaiveDate::from_ymd(1518, 11, 04),
                    guard_id: Guard(99),
                    asleep: vec![Period { begin: 36, end: 46 }],
                },
                Shift {
                    date: NaiveDate::from_ymd(1518, 11, 05),
                    guard_id: Guard(99),
                    asleep: vec![Period { begin: 45, end: 55 }],
                },
            ]
        };
    }

    #[test]
    fn test_parse_input_step1() {
        assert_eq!(
            parse_input_step1(TEST_INPUT_DATA),
            *TEST_INPUT_INTERMEDIATE_RESULT
        );
    }

    #[test]
    fn test_parse_input_step2() {
        assert_eq!(
            parse_input_step2(&*TEST_INPUT_INTERMEDIATE_RESULT),
            *TEST_INPUT_RESULT
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT_DATA), *TEST_INPUT_RESULT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*TEST_INPUT_RESULT), 240);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*TEST_INPUT_RESULT), 4455);
    }
}
