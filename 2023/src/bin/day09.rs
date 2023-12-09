#![feature(test)]
extern crate test;
use aoc_2023::*;

use itertools::Itertools;

const DAY: u8 = 9;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

pub fn solve_p1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let ns = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec();
            predict_next(&ns)
        })
        .sum()
}

fn predict_next(ns: &[i64]) -> i64 {
    if ns.iter().all(|n| *n == 0) {
        return 0;
    }
    let slopes = ns.windows(2).map(|x| x[1] - x[0]).collect_vec();
    ns.last().unwrap() + predict_next(&slopes)
}

pub fn solve_p2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let ns = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec();
            predict_prev(&ns)
        })
        .sum()
}

fn predict_prev(ns: &[i64]) -> i64 {
    if ns.iter().all(|n| *n == 0) {
        return 0;
    }
    let slopes = ns.windows(2).map(|x| x[1] - x[0]).collect_vec();
    ns[0] - predict_prev(&slopes)
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day09/sample.txt");

    #[test]
    fn test_predict_next() {
        let a = predict_next(&[0, 3, 6, 9, 12, 15]);
        assert_eq!(a, 18);

        let a = predict_next(&[1, 3, 6, 10, 15, 21]);
        assert_eq!(a, 28);

        let a = predict_next(&[10, 13, 16, 21, 30, 45]);
        assert_eq!(a, 68);
    }

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 114)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 2101499000)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 2)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 1089)
    }
}

#[cfg(test)]
mod day09_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    #[ignore]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p1(input))
    }

    #[bench]
    #[ignore]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
