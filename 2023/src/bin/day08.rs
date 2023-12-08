#![feature(test)]
extern crate test;
use aoc_2023::*;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    multi::*,
    sequence::*,
    *,
};
use nom_supreme::ParserExt;
use std::collections::*;

const DAY: u8 = 8;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct DessertMap<'a> {
    directions: Vec<Direction>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> DessertMap<'a> {
    fn new(directions: Vec<Direction>, map: HashMap<&'a str, (&'a str, &'a str)>) -> Self {
        Self { directions, map }
    }
}

fn parse_map(input: &str) -> IResult<&str, HashMap<&str, (&str, &str)>> {
    separated_list1(
        line_ending,
        separated_pair(
            alphanumeric1,
            tag(" = "),
            separated_pair(
                tag("(").precedes(alphanumeric1),
                tag(", "),
                alphanumeric1.terminated(tag(")")),
            ),
        ),
    )
    .map(|list| list.into_iter().collect())
    .parse(input)
}
fn parse(input: &str) -> IResult<&str, DessertMap> {
    separated_pair(
        alphanumeric1.terminated(line_ending).map(|l: &str| {
            l.chars()
                .map(|c| match c {
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    d => unreachable!("Impossible direction {d}"),
                })
                .collect()
        }),
        line_ending,
        parse_map,
    )
    .map(|x| DessertMap::new(x.0, x.1))
    .terminated(line_ending)
    .parse(input)
}

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input),
    );
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let (_, dm) = parse(input).unwrap();

        let mut current = "AAA";
        dm.directions
            .into_iter()
            .cycle()
            .enumerate()
            .find_map(|(index, dir)| {
                current = match dir {
                    Direction::Left => dm.map[current].0,
                    Direction::Right => dm.map[current].1,
                };
                (current == "ZZZ").then_some(index + 1)
            })
            .unwrap()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let (_, dm) = parse(input).unwrap();

        let ns = dm
            .map
            .keys()
            .cloned()
            .filter(|k| k.ends_with('A'))
            .map(|mut current| {
                dm.directions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(index, dir)| {
                        current = match dir {
                            Direction::Left => dm.map[current].0,
                            Direction::Right => dm.map[current].1,
                        };
                        current.ends_with('Z').then_some(index + 1)
                    })
                    .expect("A start node should always has a path to the end")
            })
            .collect_vec();

        lcm(&ns)
    }

    fn lcm(ns: &[usize]) -> usize {
        if ns.len() == 1 {
            return ns[0];
        }

        let a = ns[0];
        let b = lcm(&ns[1..]);
        a * b / gcd(a, b)
    }

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            return a;
        }
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day08/sample.txt");
    const SAMPLE_2: &str = include_str!("../../inputs/day08/sample_2.txt");

    #[test]
    fn test_parse_sample() {
        use Direction::*;
        let (rem, dm) = parse(SAMPLE).unwrap();
        assert_eq!(rem, "");
        assert_eq!(dm.directions, vec![Right, Left]);
        let mut em = HashMap::new();
        em.insert("AAA", ("BBB", "CCC"));
        em.insert("BBB", ("DDD", "EEE"));
        em.insert("CCC", ("ZZZ", "GGG"));
        em.insert("DDD", ("DDD", "DDD"));
        em.insert("EEE", ("EEE", "EEE"));
        em.insert("GGG", ("GGG", "GGG"));
        em.insert("ZZZ", ("ZZZ", "ZZZ"));
        assert_eq!(dm.map, em)
    }

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 2)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 19667)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE_2), 6)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 19185263738117)
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
        b.iter(|| p1::solve(input))
    }

    #[bench]
    #[ignore]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input))
    }
}
