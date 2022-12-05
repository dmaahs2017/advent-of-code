#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]
use std::fs;
use std::io::read_to_string;

pub fn read_input(day: u8) -> String {
    let f =
        fs::File::open(format!("inputs/day{:0>2}/input.txt", day)).expect("input.txt not found");
    read_to_string(f).expect("Failed to read input file to string")
}

pub fn read_sample(day: u8) -> String {
    let f =
        fs::File::open(format!("inputs/day{:0>2}/sample.txt", day)).expect("input.txt not found");
    read_to_string(f).expect("Failed to read input file to string")
}

pub trait IterOvers<T> {
    fn over_evens(self) -> impl Iterator<Item = T>;
    fn over_odds(self) -> impl Iterator<Item = T>;
}

impl<T, I> IterOvers<T> for I
where
    I: Iterator<Item = T>,
{
    fn over_evens(self) -> impl Iterator<Item = T> {
        self.enumerate()
            .filter_map(|x| if x.0 % 2 == 0 { Some(x.1) } else { None })
    }
    fn over_odds(self) -> impl Iterator<Item = T> {
        self.enumerate()
            .filter_map(|x| if x.0 % 2 != 0 { Some(x.1) } else { None })
    }
}
