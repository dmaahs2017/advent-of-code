#![feature(test)]
extern crate test;
use aoc_2022::*;

const DAY: u8 = 4;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

type Range = (usize, usize);

fn parse_input(input: &str) -> impl Iterator<Item = (Range, Range)> + '_ {
    fn parse_range(s: &str) -> Range {
        let (a, b) = s
            .split_once('-')
            .expect("Range should be separated by a dash");
        (
            a.parse().expect("Should be an integer"),
            b.parse().expect("Should be an integer"),
        )
    }

    input.lines().map(|line| {
        let (s1, s2) = line.split_once(',').expect("Each line should have a comma");
        (parse_range(s1), parse_range(s2))
    })
}

pub mod p1 {
    use super::*;
    /// In how many assignment pairs does one range fully contain the other?
    pub fn solve(input: &str) -> usize {
        parse_input(input)
            .filter(|&(a, b)| is_subset(a, b) || is_subset(b, a))
            .count()
    }

    fn is_subset(a: (usize, usize), b: (usize, usize)) -> bool {
        a.0 >= b.0 && a.1 <= b.1
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        parse_input(input)
            .filter(|&(a, b)| !is_distinct(a, b))
            .count()
    }

    fn is_distinct(a: (usize, usize), b: (usize, usize)) -> bool {
        a.1 < b.0 || b.1 < a.0
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day04/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 2)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 534)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 4)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 841)
    }
}

#[cfg(test)]
mod day04_benchmarks {
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
}
