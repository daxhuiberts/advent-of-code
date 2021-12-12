use aoctools::main;
use aoctools::IterExt;
use itertools::Itertools;
use std::collections::HashMap;

main!("day12", parse_input);

type Input = HashMap<&'static str, Vec<&'static str>>;

fn parse_input(input: &'static str) -> Input {
    input
        .lines()
        .flat_map(|line| {
            let (start, end) = line.split("-").collect_tuple().unwrap();
            if start == "start" || end == "end" {
                vec![(start, end)]
            } else if end == "start" || start == "end" {
                vec![(end, start)]
            } else {
                vec![(start, end), (end, start)]
            }
        })
        .fold_ref(HashMap::new(), |map, (start, end)| {
            map.entry(start).or_insert(vec![]).push(end)
        })
}

fn part1(input: &Input) -> usize {
    calculate_paths(input, |path, node| !path.contains(&node)).len()
}

fn part2(input: &Input) -> usize {
    calculate_paths(input, |path, node| {
        !path.contains(&node) || {
            path.iter()
                .filter(|node| node.chars().all(|c| c.is_ascii_lowercase()))
                .duplicates()
                .count()
                == 0
        }
    })
    .len()
}

fn calculate_paths(
    input: &Input,
    small_cave_check: fn(&[&str], &str) -> bool,
) -> Vec<Vec<&'static str>> {
    let mut paths = vec![];
    let mut finished_paths = vec![];
    let start: &[&str] = input.get("start").unwrap();
    for next in start {
        paths.push(vec!["start", next]);
    }
    while let Some(path) = paths.pop() {
        let next_nodes: &[&str] = input.get(path.last().unwrap()).unwrap();
        for next_node in next_nodes {
            if next_node == &"end" {
                let mut new_path = path.clone();
                new_path.push("end");
                finished_paths.push(new_path);
            } else if next_node.chars().all(|c| c.is_ascii_uppercase())
                || small_cave_check(&path, &next_node)
            {
                let mut new_path = path.clone();
                new_path.push(next_node);
                paths.push(new_path);
            }
        }
    }
    finished_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    fn p(input: &'static str) -> Input {
        parse_input(input)
    }

    const INPUT_STR: &'static str = indoc! { "
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    " };

    lazy_static! {
        static ref INPUT: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
            ("start", vec!["A", "b"]),
            ("A", vec!["c", "b", "end"]),
            ("b", vec!["A", "d", "end"]),
            ("c", vec!["A"]),
            ("d", vec!["b"])
        ]);
    }

    const INPUT2_STR: &'static str = indoc! { "
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    " };

    const INPUT3_STR: &'static str = indoc! { "
        fs-end
        he-DX
        fs-he
        start-DX
        pj-DX
        end-zg
        zg-sl
        zg-pj
        pj-he
        RW-he
        fs-DX
        pj-RW
        zg-RW
        start-pj
        he-WI
        zg-he
        pj-fs
        start-RW
    " };

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 10);
        assert_eq!(part1(&p(INPUT2_STR)), 19);
        assert_eq!(part1(&p(INPUT3_STR)), 226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 36);
        assert_eq!(part2(&p(INPUT2_STR)), 103);
        assert_eq!(part2(&p(INPUT3_STR)), 3509);
    }
}
