#![feature(test)]
extern crate test;
use aoc_2015::*;
const DAY: u8 = 04;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

fn start_with_zeroes(input: &str, n: usize) -> usize {
    (0..).find_map(|i| {
        let input = format!("{}{}", input.trim(), i);
        let hash = format!("{:x}",md5::compute(input));
        if hash.chars().take(n).all(|c| c == '0') {
            Some(i)
        } else {
            None
        }
    }).unwrap()
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        start_with_zeroes(input, 5)
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        start_with_zeroes(input, 6)
    }
}

#[cfg(test)]
mod day04_tests {
}
