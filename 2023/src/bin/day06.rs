#![feature(test)]
extern crate test;
use aoc_2023::*;

const DAY: u8 = 6;

use nom::{
    bytes::complete::take_till1,
    character::complete::{line_ending, space1},
    multi::separated_list1,
    sequence::pair,
    *,
};
use nom_supreme::ParserExt;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

fn parse_line(line: &str) -> IResult<&str, Vec<u64>> {
    take_till1(|c: char| c.is_ascii_digit())
        .precedes(separated_list1(space1, character::complete::u64))
        .terminated(line_ending)
        .parse(line)
}
fn parse(input: &str) -> IResult<&str, (Vec<u64>, Vec<u64>)> {
    pair(parse_line, parse_line).parse(input)
}

pub fn solve_p1(input: &str) -> u64 {
    let (_, (times, dists)) = parse(input).unwrap();

    times
        .iter()
        .zip(dists)
        .map(|(time, dist)| {
            let min_time_held = (0..=*time)
                .find(|time_held| time_held * (time - time_held) > dist)
                .unwrap();
            let max_time_held = time - min_time_held;
            max_time_held - min_time_held + 1
        })
        .product()
}

pub fn solve_p2(input: &str) -> u64 {
    let x = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let time = x[0];
    let dist = x[1];

    let min_time_held = (0..=time)
        .find(|time_held| time_held * (time - time_held) > dist)
        .unwrap();
    let max_time_held = time - min_time_held;

    max_time_held - min_time_held + 1
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day06/sample.txt");

    #[test]
    fn parse_sample() {
        let (rem, (time, dist)) = parse(SAMPLE).unwrap();
        dbg!(rem);
        assert!(rem.is_empty());
        assert_eq!(time, vec![7, 15, 30]);
        assert_eq!(dist, vec![9, 40, 200]);
    }

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 288)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 2374848)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 71503)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 39132886)
    }
}

#[cfg(test)]
mod day06_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p1(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
