extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;
extern crate itertools;

use std::hash::Hash;
use std::collections::HashMap;

pub trait IterExt : Iterator {
    fn group_count(self) -> HashMap<Self::Item, usize>
        where Self: Sized,
              <Self as Iterator>::Item: Eq + Hash
    {
        self.fold_ref(HashMap::new(), |map, item| {
            *map.entry(item).or_default() += 1
        })
    }

    fn fold_ref<B, F>(self, init: B, mut f: F) -> B
        where Self: Sized,
              F: FnMut(&mut B, Self::Item)
    {
        self.fold(init, |mut acc, item| {
            f(&mut acc, item);
            acc
        })
    }
}

impl<T> IterExt for T where T: Iterator { }

pub mod day1;
pub mod day2;

aoc_lib!{ year = 2018 }
