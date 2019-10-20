use std::collections::HashSet;
use std::collections::VecDeque;

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input);
    println!("{:?}", grid);

    for _ in 0..100 {
        step(&mut grid);
        println!("{:?}", grid);
    }

    grid.width * grid.height
}

fn step(grid: &mut Grid) {
    let mut visited = HashSet::new();

    for y in 0..grid.height {
        for x in 0..grid.width {
            if !visited.contains(&(x, y)) {
                let cell = grid.get(x, y);
                match cell {
                    '#' => (), // wall
                    '.' => (), // floor
                    'G' | 'E' => {
                        if let Some((new_x, new_y)) = act(grid, x, y, cell) {
                            visited.insert((new_x, new_y));
                        }
                    }
                    _ => panic!("not allowed"),
                }
            }
        }
    }

    fn act(grid: &mut Grid, x: usize, y: usize, me: char) -> Option<(usize, usize)> {
        if grid.get(x - 1, y) == opposite(me) ||
            grid.get(x + 1, y) == opposite(me) ||
            grid.get(x, y - 1) == opposite(me) ||
            grid.get(x, y + 1) == opposite(me)
        {
            attack(grid, x, y, me);
            None
        } else {
            walk(grid, x, y, me)
        }
    }

    #[allow(unused_variables)]
    fn attack(grid: &mut Grid, x: usize, y: usize, me: char) {

    }

    #[allow(unused_variables)]
    fn walk(grid: &mut Grid, x: usize, y: usize, me: char) -> Option<(usize, usize)>{
        let opposite = opposite(me);
        let directions = [(0, usize::max_value()), (usize::max_value(), 0), (1, 0), (0, 1)];
        let mut frontier = VecDeque::new();
        let mut visited = HashSet::new();

        for &(xoffset, yoffset) in &directions {
            let cell = grid.gett(apply(x, xoffset, y, yoffset));
            if cell == '.' {
                frontier.push_back((apply(x, xoffset, y, yoffset), (xoffset, yoffset)));
                visited.insert(apply(x, xoffset, y, yoffset));
            }
        }

        while let Some(((xx, yy), direction)) = frontier.pop_front() {
            for &(xoffset, yoffset) in &directions {
                if !visited.contains(&apply(xx, xoffset, yy, yoffset)) {
                    let cell = grid.gett(apply(xx, xoffset, yy, yoffset));
                    if cell == opposite {
                        let new_pos = apply(x, direction.0, y, direction.1);
                        grid.set(x, y, '.');
                        grid.sett(new_pos, me);
                        return Some(new_pos);
                    } else if cell == '.' {
                        frontier.push_back((apply(xx, xoffset, yy, yoffset), direction));
                        visited.insert(apply(xx, xoffset, yy, yoffset));
                    }
                }
            }
        }

        return None;

        fn apply(x: usize, xoffset: usize, y: usize, yoffset: usize) -> (usize, usize) {
            (x.overflowing_add(xoffset).0, y.overflowing_add(yoffset).0)
        }
    }

    fn opposite(me: char) -> char {
        match me {
            'G' => 'E',
            'E' => 'G',
            _ => panic!("not allowed")
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<char>,
}

impl Grid {
    pub fn from_str(str: &str) -> Self {
        let width = str.lines().next().unwrap().chars().count();
        let height = str.lines().count();

        let data = str.lines().flat_map(|line| {
            let line = line.chars().collect::<Vec<char>>();
            assert!(line.len() == width, "line not same width as first line");
            line
        }).collect();

        Self::new(width, height, data)
    }

    pub fn new(width: usize, height: usize, data: Vec<char>) -> Self {
        assert!(data.len() == width * height, "data len not correct");

        Self { width, height, data }
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        assert!(x < self.width, "x outside width");
        assert!(y < self.height, "y outside height");

        self.data[x + y * self.width]
    }

    pub fn gett(&self, (x, y): (usize, usize)) -> char {
        assert!(x < self.width, "x outside width");
        assert!(y < self.height, "y outside height");

        self.data[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, cell: char) {
        assert!(x < self.width, "x outside width");
        assert!(y < self.height, "y outside height");

        self.data[x + y * self.width] = cell;
    }

    pub fn sett(&mut self, (x, y): (usize, usize), cell: char) {
        assert!(x < self.width, "x outside width");
        assert!(y < self.height, "y outside height");

        self.data[x + y * self.width] = cell;
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for line in self.data.chunks(self.width) {
            for cell in line.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

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

    #[test]
    fn test_grid_step() {
        let mut grid = Grid::from_str(ROUND_0);
        println!("{:?}", grid);

        assert_eq!(format!("{:?}", grid), ROUND_0);
        step(&mut grid);
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_1);
        step(&mut grid);
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_2);
        step(&mut grid);
        println!("{:?}", grid);
        assert_eq!(format!("{:?}", grid), ROUND_3);
    }
}
