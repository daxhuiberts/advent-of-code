use itertools::Itertools;

#[aoc(day14, part1)]
pub fn part1(input: &str) -> String {
    let input: usize = input.parse().unwrap();

    let mut recipes: Vec<usize> = vec![3, 7];
    let mut elves: Vec<usize> = vec![0, 1];
    let mut len: usize = recipes.len();

    // print(&recipes, &elves);

    while len < input + 10 {
        let scores: Vec<usize> = elves.iter().map(|index| recipes[*index]).collect();

        let sum: usize = scores.iter().sum();

        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);

        len = recipes.len();

        elves = elves
            .iter()
            .zip(scores.iter())
            .map(|(current_index, score)| (current_index + score + 1) % len)
            .collect();

        // print(&recipes, &elves);
    }

    recipes[input..(input + 10)].iter().join("")
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let input_len: usize = input.len();
    let input: Vec<usize> = input
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect();

    let mut recipes: Vec<usize> = vec![3, 7];
    let mut elves: Vec<usize> = vec![0, 1];
    let mut len: usize = recipes.len();
    let mut found = None;

    // print(&recipes, &elves);

    while found.is_none() {
        let scores: Vec<usize> = elves.iter().map(|index| recipes[*index]).collect();

        let sum: usize = scores.iter().sum();

        if sum >= 10 {
            recipes.push(sum / 10);
        }
        recipes.push(sum % 10);

        len = recipes.len();

        elves = elves
            .iter()
            .zip(scores.iter())
            .map(|(current_index, score)| (current_index + score + 1) % len)
            .collect();

        // print(&recipes, &elves);

        if len > input_len {
            found = recipes[(len - input_len - 1)..len].windows(input_len).enumerate().find(|(_, window)| **window == input[..]).map(|(offset, _)| offset);
        }
    }

    println!("{:?}", found);

    len - input_len - (1 - found.unwrap())
}

#[allow(dead_code)]
fn print(recipes: &[usize], elves: &[usize]) {
    println!("{:?} - {:?}", elves, recipes);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("5"), "0124515891");
        assert_eq!(part1("9"), "5158916779");
        assert_eq!(part1("18"), "9251071085");
        assert_eq!(part1("2018"), "5941429882");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }
}
