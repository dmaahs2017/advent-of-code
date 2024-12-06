#![feature(test)]
extern crate test;
use aoc_2024::*;
use nom::{character::complete as character, multi::*, sequence::*, IResult};
use std::collections::HashMap;

const DAY: u8 = 1;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

fn parse(s: &str) -> IResult<&str, Vec<(u64, u64)>> {
    separated_list1(
        character::line_ending,
        separated_pair(character::u64, character::space1, character::u64),
    )(s)
}

pub fn solve_p1(input: &str) -> u64 {
    let (mut a, mut b): (Vec<_>, Vec<_>) = parse(input).unwrap().1.into_iter().unzip();
    a.sort();
    b.sort();

    a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum()
}

pub fn solve_p2(input: &str) -> u64 {
    let (left, right): (Vec<_>, Vec<_>) = parse(input).unwrap().1.into_iter().unzip();

    let multipliers_map =
        right
            .into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<u64, u64>, n| {
                *acc.entry(n).or_default() += 1;
                acc
            });

    left.into_iter()
        .map(|n| n * multipliers_map.get(&n).unwrap_or(&0u64))
        .sum()
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day01/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 11)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 2000468)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 31)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 18567089)
    }
}

#[cfg(test)]
mod day01_benchmarks {
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
