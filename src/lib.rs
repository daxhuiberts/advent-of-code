extern crate itertools;
extern crate regex;
extern crate chrono;

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

#[cfg(test)]
extern crate indoc;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub mod iterext;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

use iterext::IterExt;

aoc_lib!{ year = 2018 }
