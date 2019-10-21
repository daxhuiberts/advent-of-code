use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input);
    let result = grid.process();
    println!("{:?}", grid);
    result
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<Cell>,
}

impl Grid {
    pub fn from_str(str: &str) -> Self {
        let width = str.lines().next().unwrap().chars().count();
        let height = str.lines().count();

        let data = str.lines().flat_map(|line| {
            let line = line.chars().map(Cell::from_char).collect::<Vec<Cell>>();
            assert!(line.len() == width, "line not same width as first line");
            line
        }).collect();

        Self::new(width, height, data)
    }

    pub fn new(width: usize, height: usize, data: Vec<Cell>) -> Self {
        assert!(data.len() == width * height, "data len not correct");

        Self { width, height, data }
    }

    pub fn process(&mut self) -> usize {
        for i in 0..1000 {
            if !self.step() {
                return i * self.count_health()
            }
        }

        0
    }

    pub fn step(&mut self) -> bool {
        let mut visited = HashSet::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if !visited.contains(&(x, y)) {
                    let cell = self.get(x, y);
                    match cell {
                        Cell::Wall => (),
                        Cell::Floor => (),
                        Cell::Creature { .. } => {
                            if self.is_done() {
                                return false;
                            }

                            let new_pos = self.act(x, y, cell);
                            visited.insert(new_pos);
                        }
                    }
                }
            }
        }

        true
    }

    pub fn is_done(&self) -> bool {
        [CreatureRace::Elf, CreatureRace::Gnome].iter().any(|race| {
            !self.data.iter().any(|cell| {
                match cell {
                    Cell::Creature { race: cell_race, .. } => race == cell_race,
                    _ => false
                }
            })
        })
    }

    pub fn count_health(&self) -> usize {
        self.data.iter().map(|cell| {
            match cell {
                Cell::Creature { health, .. } => *health,
                _ => 0
            }
        }).sum()
    }

    fn act(&mut self, x: usize, y: usize, me: Cell) -> (usize, usize) {
        let (new_x, new_y) = if !self.enemy_near(x, y, me) {
            self.walk(x, y, me)
        } else {
            (x, y)
        };

        if self.enemy_near(new_x, new_y, me) {
            self.attack(new_x, new_y, me);
        };

        (new_x, new_y)
    }

    #[allow(unused_variables)]
    fn attack(&mut self, x: usize, y: usize, me: Cell) {
        let positions = vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
        let mut possible_enemies: Vec<(usize, usize, usize)> = positions.into_iter().filter_map(|(xx, yy)| {
            let other = self.get(xx, yy);
            if other.is_enemy_of(&me) {
                if let Cell::Creature { health, .. } = other {
                    Some((xx, yy, health))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();
        possible_enemies.sort_by_key(|a| a.2 );

        if let Some((xx, yy, _)) = possible_enemies.first() {
            let enemy = self.get(*xx, *yy);
            if let Cell::Creature { race, health } = enemy {
                if health < 3 {
                    self.set(*xx, *yy, Cell::Floor);
                } else {
                    self.set(*xx, *yy, Cell::Creature { race, health: health - 3 } );
                }
            } else {
                panic!("NOT POSSIBLE!!")
            }
        }
    }

    fn enemy_near(&self, x: usize, y: usize, me: Cell) -> bool {
        let positions = vec![(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)];
        positions.into_iter().any(|(xx, yy)|
            self.get(xx, yy).is_enemy_of(&me)
        )
    }

    fn walk(&mut self, x: usize, y: usize, me: Cell) -> (usize, usize) {
        let directions = [(0, usize::max_value()), (usize::max_value(), 0), (1, 0), (0, 1)];
        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();

        for &(xoffset, yoffset) in &directions {
            let cell = self.get_with_offset(x, xoffset, y, yoffset);
            if cell == Cell::Floor {
                frontier.push_back((Self::apply(x, xoffset, y, yoffset), (xoffset, yoffset)));
                visited.insert(Self::apply(x, xoffset, y, yoffset));
            }
        }

        while let Some(((xx, yy), direction)) = frontier.pop_front() {
            for &(xoffset, yoffset) in &directions {
                if !visited.contains(&Self::apply(xx, xoffset, yy, yoffset)) {
                    let cell = self.get_with_offset(xx, xoffset, yy, yoffset);
                    if cell.is_enemy_of(&me) {
                        let new_pos = Self::apply(x, direction.0, y, direction.1);
                        self.set(x, y, Cell::Floor);
                        self.set(new_pos.0, new_pos.1, me);
                        return new_pos;
                    } else if cell == Cell::Floor {
                        frontier.push_back((Self::apply(xx, xoffset, yy, yoffset), direction));
                        visited.insert(Self::apply(xx, xoffset, yy, yoffset));
                    }
                }
            }
        }

        (x, y)
    }

    fn apply(x: usize, xoffset: usize, y: usize, yoffset: usize) -> (usize, usize) {
        (x.overflowing_add(xoffset).0, y.overflowing_add(yoffset).0)
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.width, "x outside width");
        assert!(y < self.height, "y outside height");

        self.data[x + y * self.width]
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        assert!(x < self.width, "x outside width");
        assert!(y < self.height, "y outside height");

        self.data[x + y * self.width] = cell;
    }

    fn get_with_offset(&self, x: usize, xoffset: usize, y: usize, yoffset: usize) -> Cell {
        let (xx, yy) = Self::apply(x, xoffset, y, yoffset);
        self.get(xx, yy)
    }

    #[allow(dead_code)]
    fn set_with_offset(&mut self, x: usize, xoffset: usize, y: usize, yoffset: usize, cell: Cell) {
        let (xx, yy) = Self::apply(x, xoffset, y, yoffset);
        self.set(xx, yy, cell)
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.data.chunks(self.width) {
            for cell in line.iter() {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cell {
    Wall,
    Floor,
    Creature {
        race: CreatureRace,
        health: usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum CreatureRace {
    Elf,
    Gnome
}

impl Cell {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Cell::Wall,
            '.' => Cell::Floor,
            'E' => Cell::Creature { race: CreatureRace::Elf, health: 200 },
            'G' => Cell::Creature { race: CreatureRace::Gnome, health: 200 },
            _ => panic!("invalid character")
        }
    }

    pub fn is_enemy_of(&self, other: &Cell) -> bool {
        match (self, other) {
            (Cell::Creature { race, .. }, Cell::Creature { race: other_race, .. }) => race != other_race,
            _ => false
        }
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Cell::Wall => "#",
            Cell::Floor => ".",
            Cell::Creature { race, .. } => match race {
                CreatureRace::Elf => "E",
                CreatureRace::Gnome => "G",
            },
        })?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_grid_move() {
        const ROUND_0: &'static str = indoc!(
            "
            #########
            #G..G..G#
            #.......#
            #.......#
            #G..E..G#
            #.......#
            #.......#
            #G..G..G#
            #########
        "
        );

        const ROUND_1: &'static str = indoc!(
            "
            #########
            #.G...G.#
            #...G...#
            #...E..G#
            #.G.....#
            #.......#
            #G..G..G#
            #.......#
            #########
        "
        );

        const ROUND_2: &'static str = indoc!(
            "
            #########
            #..G.G..#
            #...G...#
            #.G.E.G.#
            #.......#
            #G..G..G#
            #.......#
            #.......#
            #########
        "
        );

        const ROUND_3: &'static str = indoc!(
            "
            #########
            #.......#
            #..GGG..#
            #..GEG..#
            #G..G...#
            #......G#
            #.......#
            #.......#
            #########
        "
        );

        let mut grid = Grid::from_str(ROUND_0);
        println!("{:?}", grid);

        assert_eq!(format!("{:?}", grid), ROUND_0);
        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_1);
        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_2);
        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_3);
        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_3);
    }

    #[test]
    fn test_grid_attack() {
        const ROUND_0: &'static str = indoc!(
            "
            #######
            #.G...#
            #...EG#
            #.#.#G#
            #..G#E#
            #.....#
            #######
        "
        );

        const ROUND_1: &'static str = indoc!(
            "
            #######
            #..G..#
            #...EG#
            #.#G#G#
            #...#E#
            #.....#
            #######
        "
        );

        const ROUND_2: &'static str = indoc!(
            "
            #######
            #...G.#
            #..GEG#
            #.#.#G#
            #...#E#
            #.....#
            #######
        "
        );

        const ROUND_23: &'static str = indoc!(
            "
            #######
            #...G.#
            #..G.G#
            #.#.#G#
            #...#E#
            #.....#
            #######
        "
        );

        const ROUND_24: &'static str = indoc!(
            "
            #######
            #..G..#
            #...G.#
            #.#G#G#
            #...#E#
            #.....#
            #######
        "
        );

        const ROUND_25: &'static str = indoc!(
            "
            #######
            #.G...#
            #..G..#
            #.#.#G#
            #..G#E#
            #.....#
            #######
        "
        );

        const ROUND_26: &'static str = indoc!(
            "
            #######
            #G....#
            #.G...#
            #.#.#G#
            #...#E#
            #..G..#
            #######
        "
        );

        const ROUND_27: &'static str = indoc!(
            "
            #######
            #G....#
            #.G...#
            #.#.#G#
            #...#E#
            #...G.#
            #######
        "
        );

        const ROUND_28: &'static str = indoc!(
            "
            #######
            #G....#
            #.G...#
            #.#.#G#
            #...#E#
            #....G#
            #######
        "
        );

        const ROUND_47: &'static str = indoc!(
            "
            #######
            #G....#
            #.G...#
            #.#.#G#
            #...#.#
            #....G#
            #######
        "
        );

        let mut grid = Grid::from_str(ROUND_0);
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_0);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_1);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_2);

        for _ in 0..20 { grid.step(); };
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_2);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_23);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_24);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_25);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_26);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_27);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_28);

        for _ in 0..18 { grid.step(); };
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_28);

        grid.step();
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_47);
    }

    #[test]
    fn test_grid_result() {
        const COMBAT_1_BEGIN: &'static str = indoc!(
            "
            #######
            #.G...#
            #...EG#
            #.#.#G#
            #..G#E#
            #.....#
            #######
        "
        );

        const COMBAT_1_END: &'static str = indoc!(
            "
            #######
            #G....#
            #.G...#
            #.#.#G#
            #...#.#
            #....G#
            #######
        "
        );

        let mut grid = Grid::from_str(COMBAT_1_BEGIN);
        let result = grid.process();
        assert_eq!(format!("{:?}", grid), COMBAT_1_END);
        assert_eq!(result, 27730);

        const COMBAT_2_BEGIN: &'static str = indoc!(
            "
            #######
            #G..#E#
            #E#E.E#
            #G.##.#
            #...#E#
            #...E.#
            #######
        "
        );

        const COMBAT_2_END: &'static str = indoc!(
            "
            #######
            #...#E#
            #E#...#
            #.E##.#
            #E..#E#
            #.....#
            #######
        "
        );

        let mut grid = Grid::from_str(COMBAT_2_BEGIN);
        let result = grid.process();
        assert_eq!(format!("{:?}", grid), COMBAT_2_END);
        assert_eq!(result, 36334);

        const COMBAT_3_BEGIN: &'static str = indoc!(
            "
            #######
            #E..EG#
            #.#G.E#
            #E.##E#
            #G..#.#
            #..E#.#
            #######
        "
        );

        const COMBAT_3_END: &'static str = indoc!(
            "
            #######
            #.E.E.#
            #.#E..#
            #E.##.#
            #.E.#.#
            #...#.#
            #######
        "
        );

        let mut grid = Grid::from_str(COMBAT_3_BEGIN);
        let result = grid.process();
        assert_eq!(format!("{:?}", grid), COMBAT_3_END);
        assert_eq!(result, 39514);

        const COMBAT_4_BEGIN: &'static str = indoc!(
            "
            #######
            #E.G#.#
            #.#G..#
            #G.#.G#
            #G..#.#
            #...E.#
            #######
        "
        );

        const COMBAT_4_END: &'static str = indoc!(
            "
            #######
            #G.G#.#
            #.#G..#
            #..#..#
            #...#G#
            #...G.#
            #######
        "
        );

        let mut grid = Grid::from_str(COMBAT_4_BEGIN);
        let result = grid.process();
        assert_eq!(format!("{:?}", grid), COMBAT_4_END);
        assert_eq!(result, 27755);

        const COMBAT_5_BEGIN: &'static str = indoc!(
            "
            #######
            #.E...#
            #.#..G#
            #.###.#
            #E#G#G#
            #...#G#
            #######
        "
        );

        const COMBAT_5_END: &'static str = indoc!(
            "
            #######
            #.....#
            #.#G..#
            #.###.#
            #.#.#.#
            #G.G#G#
            #######
        "
        );

        let mut grid = Grid::from_str(COMBAT_5_BEGIN);
        let result = grid.process();
        assert_eq!(format!("{:?}", grid), COMBAT_5_END);
        assert_eq!(result, 28944);

        const COMBAT_6_BEGIN: &'static str = indoc!(
            "
            #########
            #G......#
            #.E.#...#
            #..##..G#
            #...##..#
            #...#...#
            #.G...G.#
            #.....G.#
            #########
        "
        );

        const COMBAT_6_END: &'static str = indoc!(
            "
            #########
            #.G.....#
            #G.G#...#
            #.G##...#
            #...##..#
            #.G.#...#
            #.......#
            #.......#
            #########
        "
        );

        let mut grid = Grid::from_str(COMBAT_6_BEGIN);
        let result = grid.process();
        assert_eq!(format!("{:?}", grid), COMBAT_6_END);
        assert_eq!(result, 18740);
    }
}
