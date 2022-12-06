#![feature(test, array_windows)]
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

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        input
            .trim()
            .as_bytes()
            .windows(4)
            .enumerate()
            .find_map(|(i, win)| {
                let set = win.iter().collect::<HashSet<_>>();
                if set.len() == 4 {
                    Some(i + 4)
                } else {
                    None
                }
            })
            .unwrap()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        input
            .trim()
            .as_bytes()
            .windows(14)
            .enumerate()
            .find_map(|(i, win)| {
                let set = win.iter().collect::<HashSet<_>>();
                if set.len() == 14 {
                    Some(i + 14)
                } else {
                    None
                }
            })
            .unwrap()
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
    #[ignore]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p1::solve(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input))
    }
}
