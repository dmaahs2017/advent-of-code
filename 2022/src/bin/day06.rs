#![feature(test, iter_collect_into)]
extern crate test;
use aoc_2022::*;
use std::collections::HashSet;

const DAY: u8 = 6;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

fn start_of_unique_window(input: &str, window_size: usize) -> usize {
    input
        .trim()
        .as_bytes()
        .windows(window_size)
        .position(|win| win.iter().collect::<HashSet<_>>().len() == window_size)
        .unwrap()
        + window_size
}

fn start_of_unique_window_optimized(input: &str, window_size: usize) -> usize {
    let mut set: HashSet<u8> = HashSet::new();
    input
        .trim()
        .as_bytes()
        .windows(window_size)
        .position(|win| {
            set.clear();
            win.iter().collect_into(&mut set).len() == window_size
        })
        .unwrap()
        + window_size
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        start_of_unique_window(input, 4)
    }

    pub fn solve_optimized(input: &str) -> usize {
        start_of_unique_window_optimized(input, 4)
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        start_of_unique_window(input, 14)
    }

    pub fn solve_optimized(input: &str) -> usize {
        start_of_unique_window_optimized(input, 14)
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day06/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 11)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 1198)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 26)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 3120)
    }
}

#[cfg(test)]
mod day06_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p1::solve(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input))
    }

    #[bench]
    fn bench_optimized_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p1::solve_optimized(input))
    }

    #[bench]
    fn bench_optimized_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve_optimized(input))
    }
}
