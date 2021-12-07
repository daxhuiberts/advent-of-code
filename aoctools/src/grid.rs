use std::iter::{Skip, StepBy, Take};
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub type LineIterator<'a, T> = Take<StepBy<Skip<Iter<'a, T>>>>;

impl<T: Copy + std::fmt::Debug> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            data: Vec::with_capacity(width * height),
        }
    }

    pub fn new_with_data(width: usize, height: usize, data: Vec<T>) -> Self {
        assert_eq!(data.len(), width * height);
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn first(&self) -> T {
        self.data[0]
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn get_mut(&mut self, (x, y): (usize, usize)) -> &mut T {
        assert!(x < self.width && y < self.height);
        &mut self.data[x + y * self.width]
    }

    pub fn rows(&self) -> impl Iterator<Item = LineIterator<'_, T>> + '_ {
        (0..self.height).map(|offset| {
            self.data
                .iter()
                .skip(offset * self.width)
                .step_by(1)
                .take(self.width)
        })
    }

    pub fn cols(&self) -> impl Iterator<Item = LineIterator<'_, T>> + '_ {
        (0..self.width).map(|offset| {
            self.data
                .iter()
                .skip(offset)
                .step_by(self.height)
                .take(self.height)
        })
    }

    pub fn cell_with_cords(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(index, cell)| ((index % self.width, index / self.width), cell))
    }

    pub fn cell_with_cords_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(index, cell)| ((index % self.width, index / self.width), cell))
    }

    const ADJACENT_FOUR: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    const ADJACENT_EIGHT: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn adjacent_values(
        &self,
        (x, y): (usize, usize),
        diagonal: bool,
    ) -> impl Iterator<Item = ((usize, usize), T)> + '_ {
        assert!(x < self.width && y < self.height);
        let adjacent = if diagonal {
            &Self::ADJACENT_EIGHT[..]
        } else {
            &Self::ADJACENT_FOUR[..]
        };
        adjacent.iter().filter_map(move |&(xoffset, yoffset)| {
            if !(xoffset == -1 && x == 0)
                && !(xoffset == 1 && x == self.width - 1)
                && !(yoffset == -1 && y == 0)
                && !(yoffset == 1 && y == self.height - 1)
            {
                let matchx = (x as isize + xoffset) as usize;
                let matchy = (y as isize + yoffset) as usize;
                let value = self.data[matchx + matchy * self.width];
                Some(((matchx, matchy), value))
            } else {
                None
            }
        })
    }
}
