use aoctools::grid::Grid;
use aoctools::main;

main!("day04", parse_input);

type Card = Grid<u32>;

#[derive(Debug, PartialEq, Eq)]
struct Game {
    draws: Vec<u32>,
    cards: Vec<Card>,
}

fn parse_input(input: &str) -> Game {
    let mut paragraphs = input.split("\n\n");
    let draws = paragraphs
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let cards = paragraphs
        .map(|card| {
            let data = card
                .split('\n')
                .flat_map(|row| {
                    row.split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect();
            Card::new_with_data(5, 5, data)
        })
        .collect();
    Game {
        draws: draws,
        cards: cards,
    }
}

fn part1(game: &Game) -> u32 {
    let (card, draws) = (1..game.draws.len())
        .find_map(|draw_count| {
            let draws = &game.draws[0..draw_count];
            game.cards
                .iter()
                .find(|&card| {
                    card.rows()
                        .chain(card.cols())
                        .any(|mut line| line.all(|&number| draws.contains(&number)))
                })
                .map(|card| (card, draws))
        })
        .unwrap();
    let sum_of_all_unmarked_numbers: u32 = card.data().iter().filter(|x| !draws.contains(x)).sum();
    sum_of_all_unmarked_numbers * draws.last().unwrap()
}

fn part2(_input: &Game) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use lazy_static::lazy_static;

    const INPUT_STR: &'static str = indoc! { "
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    " };

    lazy_static! {
        static ref INPUT: Game = {
            Game {
                draws: vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1,
                ],
                cards: vec![
                    Card::new_with_data(
                        5,
                        5,
                        vec![
                            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5,
                            1, 12, 20, 15, 19,
                        ],
                    ),
                    Card::new_with_data(
                        5,
                        5,
                        vec![
                            3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4,
                            14, 21, 16, 12, 6,
                        ],
                    ),
                    Card::new_with_data(
                        5,
                        5,
                        vec![
                            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6,
                            5, 2, 0, 12, 3, 7,
                        ],
                    ),
                ],
            }
        };
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input(INPUT_STR), *INPUT);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&*INPUT), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&*INPUT), 0);
    }
}
