#![feature(test)]
extern crate test;
use aoc_2024::*;
use glam::IVec2;
use itertools::Itertools;
use nom::{bytes::complete as bytes, character::complete as character, multi::*, IResult};
use nom_locate::LocatedSpan;
use std::collections::HashMap;
use tracing::info;

const DAY: u8 = 8;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

type Span<'a> = LocatedSpan<&'a str>;
type NodeMap = HashMap<char, Vec<IVec2>>;

fn parse_next_char(s: Span) -> IResult<Span, (char, IVec2)> {
    let (s, _) = bytes::take_while(|c| c == '.' || c == '\n')(s)?;
    let (rest, c) = character::anychar(s)?;
    let p = IVec2::new(s.location_line() as i32 - 1, s.get_column() as i32 - 1);
    Ok((rest, (c, p)))
}

fn parse(s: Span) -> IResult<Span, NodeMap> {
    fold_many1(parse_next_char, NodeMap::new, |mut acc, (c, p)| {
        acc.entry(c).or_default().push(p);
        acc
    })(s)
}

#[tracing::instrument(skip(input))]
pub fn solve_p1(input: &str) -> usize {
    let node_map = parse(input.into()).unwrap().1;
    let width = input.lines().next().unwrap().len() as i32 - 1;
    let height = input.lines().count() as i32 - 1;
    info!(width, height);

    node_map
        .into_values()
        .flat_map(|nodes| {
            nodes
                .into_iter()
                .combinations(2)
                .flat_map(|xs| {
                    let d = xs[1] - xs[0];
                    [xs[0] - d, xs[1] + d]
                })
                .filter(|an| (0..=width).contains(&an.x) && (0..=height).contains(&an.y))
        })
        .unique()
        .count()
}

pub fn solve_p2(input: &str) -> usize {
    let node_map = parse(input.into()).unwrap().1;
    let width = input.lines().next().unwrap().len() as i32 - 1;
    let height = input.lines().count() as i32 - 1;
    info!(width, height);

    node_map
        .into_values()
        .flat_map(|nodes| {
            nodes
                .into_iter()
                .combinations(2)
                .flat_map(|xs| {
                    let d = xs[0] - xs[1];
                    let mut ans = vec![];

                    let mut a = xs[0];
                    while (0..=width).contains(&a.x) && (0..=height).contains(&a.y) {
                        a -= d;
                        ans.push(a);
                    }

                    let mut b = xs[1];
                    while (0..=width).contains(&b.x) && (0..=height).contains(&b.y) {
                        b += d;
                        ans.push(b);
                    }
                    ans
                })
                .filter(|an| (0..=width).contains(&an.x) && (0..=height).contains(&an.y))
        })
        .unique()
        .count()
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day08/sample.txt");

    #[test_log::test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 14)
    }

    #[test_log::test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 332)
    }

    #[test_log::test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 34)
    }

    #[test_log::test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 1174)
    }
}

#[cfg(test)]
mod day08_benchmarks {
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
