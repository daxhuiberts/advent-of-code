#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut grid = Grid::from_str(input);
    step(&mut grid);
    println!("{:?}", grid);

    grid.width * grid.height
}

fn step(grid: &mut Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            match cell {
                '#' => (), // wall
                '.' => (), // floor
                'G' => (), // gnome
                'E' => (), // Self
                _ => panic!("not allowed"),
            }
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
        r"
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
        r"
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
        r"
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
        r"
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

        assert_eq!(format!("{:?}", grid), ROUND_0);
        step(&mut grid);
        assert_eq!(format!("{:?}", grid), ROUND_1);
        step(&mut grid);
        assert_eq!(format!("{:?}", grid), ROUND_2);
        step(&mut grid);
        assert_eq!(format!("{:?}", grid), ROUND_3);
    }
}
