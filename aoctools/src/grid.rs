use std::iter::{Skip, StepBy, Take};
use std::slice::Iter;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub type LineIterator<'a, T> = Take<StepBy<Skip<Iter<'a, T>>>>;

impl<T: Copy> Grid<T> {
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
}
