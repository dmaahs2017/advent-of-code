#![feature(test)]
extern crate test;
use aoc_2024::*;
use nom::{
    bytes::complete as bytes, character::complete as character, multi::*, sequence::*, IResult,
    Parser,
};
use nom_supreme::ParserExt;
use std::collections::{HashMap, HashSet};

const DAY: u8 = 5;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

type Updates = Vec<u32>;

fn parse(s: &str) -> IResult<&str, (HashMap<u32, Vec<u32>>, Vec<Updates>)> {
    separated_pair(
        separated_list1(
            character::line_ending,
            separated_pair(character::u32, bytes::tag("|"), character::u32),
        )
        .map(|pairs| {
            pairs.into_iter().fold(HashMap::new(), |mut acc, (a, b)| {
                acc.entry(b).or_insert(vec![]).push(a);
                acc
            })
        }),
        character::line_ending.terminated(character::line_ending),
        separated_list1(
            character::line_ending,
            separated_list1(bytes::tag(","), character::u32),
        ),
    )(s)
}

pub fn solve_p1(input: &str) -> u32 {
    let (map, update_list) = parse(input).unwrap().1;

    update_list
        .into_iter()
        .filter_map(|updates| {
            let mut well_ordered = true;
            let mut disallowed = HashSet::new();

            for update in updates.iter() {
                if disallowed.contains(update) {
                    well_ordered = false;
                    break;
                }
                if let Some(xs) = map.get(update) {
                    for x in xs {
                        disallowed.insert(x);
                    }
                }
            }

            if well_ordered {
                Some(updates[updates.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_p2(input: &str) -> u32 {
    let (map, update_list) = parse(input).unwrap().1;

    update_list
        .into_iter()
        .filter_map(|mut updates| {
            let mut well_ordered = true;
            let mut disallowed = HashSet::new();

            for update in updates.iter() {
                if disallowed.contains(update) {
                    well_ordered = false;
                    break;
                }
                if let Some(xs) = map.get(update) {
                    for x in xs {
                        disallowed.insert(x);
                    }
                }
            }

            if !well_ordered {
                updates.sort_by(|a, b| {
                    let a_comes_before = map.get(a).cloned().unwrap_or_default();
                    let b_comes_before = map.get(b).cloned().unwrap_or_default();

                    if a_comes_before.contains(b) {
                        std::cmp::Ordering::Less
                    } else if b_comes_before.contains(a) {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Equal
                    }
                });
                Some(updates[updates.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day05/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 143)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 6951)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 123)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 4121)
    }
}

#[cfg(test)]
mod day05_benchmarks {
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
