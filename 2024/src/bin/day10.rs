#![feature(test)]
extern crate test;
use aoc_2024::*;

use tracing::info;

use glam::{ivec2, IVec2};
use nom::{bytes::complete as bytes, character::complete as character, multi::*, IResult, Parser};
use nom_locate::LocatedSpan;
use nom_supreme::ParserExt;
use std::collections::{HashMap, HashSet};

const DAY: u8 = 10;

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

type HeightMap = HashMap<IVec2, i32>;

fn height_with_location(s: Span) -> IResult<Span, (IVec2, i32)> {
    let p = ivec2(s.get_column() as i32 - 1, s.location_line() as i32 - 1);

    let (rest, number_span) = bytes::take(1usize)(s)?;

    let (_, x) = character::i32(number_span)?;
    Ok((rest, (p, x)))
}
fn parse(s: Span) -> IResult<Span, HeightMap> {
    let (rest, xs) = separated_list1(character::line_ending, many1(height_with_location))(s)?;
    let hm = xs.into_iter().flatten().collect();
    Ok((rest, hm))
}

#[tracing::instrument(skip(input))]
pub fn solve_p1(input: &str) -> usize {
    let hm = parse(input.into()).unwrap().1;
    let width = hm.keys().map(|k| k.x).max().unwrap();
    let height = hm.keys().map(|k| k.y).max().unwrap();
    let bounds = ivec2(width, height);

    hm.iter()
        .filter_map(|node| {
            if *node.1 == 0 {
                Some(score_node_by_peaks_reached(
                    *node.0,
                    &hm,
                    0,
                    &bounds,
                    &mut HashMap::new(),
                ))
            } else {
                None
            }
        })
        .sum()
}

const OFFSETS: [IVec2; 4] = [ivec2(0, 1), ivec2(0, -1), ivec2(1, 0), ivec2(-1, 0)];
fn score_node_by_peaks_reached(
    node: IVec2,
    heightmap: &HeightMap,
    score: usize,
    bounds: &IVec2,
    memo: &mut HashMap<IVec2, usize>,
) -> usize {
    if let Some(s) = memo.get(&node) {
        return *s;
    }
    if heightmap[&node] == 9 {
        memo.insert(node, 1);
        return 1;
    }

    OFFSETS
        .into_iter()
        .filter_map(|o| {
            let adj_node = node + o;
            if !(0..=bounds.x).contains(&adj_node.x)
                || !(0..=bounds.y).contains(&adj_node.y)
                || heightmap[&adj_node] - heightmap[&node] != 1
            {
                return None;
            }

            let s = score_node_by_peaks_reached(node + o, heightmap, score, bounds, memo);
            memo.insert(node + o, score);
            Some(s)
        })
        .sum()
}

fn score_node_by_forks(
    node: IVec2,
    heightmap: &HeightMap,
    bounds: &IVec2,
    memo: &mut HashMap<IVec2, usize>,
) -> usize {
    if let Some(s) = memo.get(&node) {
        return *s;
    }
    if heightmap[&node] == 9 {
        memo.insert(node, 1);
        return 1;
    }

    OFFSETS
        .into_iter()
        .filter_map(|o| {
            let adj_node = node + o;
            if !(0..=bounds.x).contains(&adj_node.x)
                || !(0..=bounds.y).contains(&adj_node.y)
                || heightmap[&adj_node] - heightmap[&node] != 1
            {
                return None;
            }

            let s = score_node_by_forks(node + o, heightmap, bounds, memo);
            memo.insert(node + o, s);
            Some(s)
        })
        .sum()
}

#[tracing::instrument(skip(input))]
pub fn solve_p2(input: &str) -> usize {
    let hm = parse(input.into()).unwrap().1;
    let width = hm.keys().map(|k| k.x).max().unwrap();
    let height = hm.keys().map(|k| k.y).max().unwrap();
    let bounds = ivec2(width, height);

    hm.iter()
        .filter_map(|node| {
            if *node.1 == 0 {
                Some(score_node_by_forks(
                    *node.0,
                    &hm,
                    &bounds,
                    &mut HashMap::new(),
                ))
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day10/sample.txt");

    #[test_log::test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 36)
    }

    #[test_log::test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 629)
    }

    #[test_log::test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 81)
    }

    #[test_log::test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 1242)
    }
}

#[cfg(test)]
mod day10_benchmarks {
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
