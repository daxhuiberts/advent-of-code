use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Begin { guard_id: usize },
    Sleep,
    Awake,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    action: Action,
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Record> {
    let regex_line = Regex::new(r"\A\[1518-(\d\d)-(\d\d) (23|00):(\d\d)\] (.+)\z").unwrap();
    let regex_action_begin = Regex::new(r"\AGuard #(\d+) begins shift\z").unwrap();

    let mut lines = input.lines().collect_vec();
    lines.sort();

    lines.iter().map(|line| {
        let captures = regex_line.captures(line).unwrap();

        let month = captures.get(1).unwrap().as_str().parse().unwrap();
        let day = captures.get(2).unwrap().as_str().parse().unwrap();
        let hour = captures.get(3).unwrap().as_str().parse().unwrap();
        let minute = captures.get(4).unwrap().as_str().parse().unwrap();
        let action_str = captures.get(5).unwrap().as_str();

        let action = regex_action_begin.captures(action_str).map(|captures| {
                let guard_id = captures.get(1).unwrap().as_str().parse().unwrap();
                Action::Begin { guard_id }
            }).unwrap_or_else(|| {
                match action_str {
                    "falls asleep" => Action::Sleep,
                    "wakes up" => Action::Awake,
                    _ => panic!("invalid action str")
                }
            });

        Record { month, day, hour, minute, action }
    }).collect_vec()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    // Use the test data from the description, but mess up chronological ordering.
    const TEST_INPUT_DATA: &'static str = indoc!("
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
    ");

    const TEST_INPUT_RESULT: [Record; 17] = [
        Record { month: 11, day: 01, hour: 00, minute: 00, action: Action::Begin { guard_id: 10 } },
        Record { month: 11, day: 01, hour: 00, minute: 05, action: Action::Sleep },
        Record { month: 11, day: 01, hour: 00, minute: 25, action: Action::Awake },
        Record { month: 11, day: 01, hour: 00, minute: 30, action: Action::Sleep },
        Record { month: 11, day: 01, hour: 00, minute: 55, action: Action::Awake },
        Record { month: 11, day: 01, hour: 23, minute: 58, action: Action::Begin { guard_id: 99 } },
        Record { month: 11, day: 02, hour: 00, minute: 40, action: Action::Sleep },
        Record { month: 11, day: 02, hour: 00, minute: 50, action: Action::Awake },
        Record { month: 11, day: 03, hour: 00, minute: 05, action: Action::Begin { guard_id: 10 } },
        Record { month: 11, day: 03, hour: 00, minute: 24, action: Action::Sleep },
        Record { month: 11, day: 03, hour: 00, minute: 29, action: Action::Awake },
        Record { month: 11, day: 04, hour: 00, minute: 02, action: Action::Begin { guard_id: 99 } },
        Record { month: 11, day: 04, hour: 00, minute: 36, action: Action::Sleep },
        Record { month: 11, day: 04, hour: 00, minute: 46, action: Action::Awake },
        Record { month: 11, day: 05, hour: 00, minute: 03, action: Action::Begin { guard_id: 99 } },
        Record { month: 11, day: 05, hour: 00, minute: 45, action: Action::Sleep },
        Record { month: 11, day: 05, hour: 00, minute: 55, action: Action::Awake },
    ];

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(TEST_INPUT_DATA), TEST_INPUT_RESULT);
    }
}
