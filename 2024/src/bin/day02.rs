#![feature(test)]
#![feature(array_windows)]
extern crate test;
use aoc_2024::*;
use nom::{character::complete as character, multi::*, IResult};

const DAY: u8 = 2;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(
        character::line_ending,
        separated_list1(character::space1, character::i64),
    )(s)
}

pub fn solve_p1(input: &str) -> usize {
    let levels = parse(input).unwrap().1;

    levels
        .into_iter()
        .filter(|level| {
            let differences = diffs(level);
            is_safe(&differences)
        })
        .count()
}

pub fn solve_p2(input: &str) -> usize {
    let levels = parse(input).unwrap().1;

    levels
        .into_iter()
        .filter(|level| {
            let differences = diffs(level);
            is_safe(&differences)
                || (0..level.len()).into_iter().any(|i| {
                    let mut level = level.clone();
                    level.remove(i);
                    let differences = diffs(&level);
                    is_safe(&differences)
                })
        })
        .count()
}

fn diffs(level: &Vec<i64>) -> Vec<i64> {
    level
        .array_windows::<2>()
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>()
}

fn is_safe(v: &Vec<i64>) -> bool {
    v.iter().all(|&n| (n > 0 && n <= 3) || (n < 0 && n >= -3))
        && (v.iter().all(|&n| n > 0) || v.iter().all(|&n| n < 0))
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day02/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 2)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 598)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 4)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 634)
    }
}

#[cfg(test)]
mod day02_benchmarks {
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
