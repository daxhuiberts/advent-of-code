#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input);
    println!("{:?}", grid);

    grid.width * grid.height
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
            if line.len() != width {
                panic!("line not same width as first line");
            }
            line
        }).collect();

        Self::new(width, height, data)
    }

    pub fn new(width: usize, height: usize, data: Vec<char>) -> Self {
        if data.len() != width * height {
            panic!("data len not correct");
        }

        Self { width, height, data }
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

    #[test]
    fn test_grid_from_str() {
        Grid::from_str(ROUND_0);
    }
}
