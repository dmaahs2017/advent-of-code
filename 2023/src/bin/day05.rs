#![feature(test)]
extern crate test;
use aoc_2023::*;

use indicatif::ProgressIterator;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1},
    multi::separated_list1,
    sequence::pair,
    *,
};
use nom_supreme::ParserExt;
use rayon::prelude::*;

const DAY: u8 = 5;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

#[derive(Debug, PartialEq)]
struct Range {
    lower: u64,
    upper: u64,
    offset: i64,
}

#[derive(Debug, PartialEq)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(ranges: Vec<Range>) -> Self {
        Self { ranges }
    }

    fn get(&self, k: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| {
                if range.lower <= k && k < range.upper {
                    Some((k as i64 + range.offset) as u64)
                } else {
                    None
                }
            })
            .unwrap_or(k)
    }
}

#[derive(Debug)]
struct Data {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

impl Data {
    fn get_seed_location(&self, mut seed: u64) -> u64 {
        for map in &self.maps {
            seed = map.get(seed);
        }
        seed
    }
}

fn seeds(input: &str) -> IResult<&str, Vec<u64>> {
    take_until(":")
        .precedes(tag(":"))
        .precedes(space1)
        .precedes(separated_list1(space1, character::complete::u64))
        .terminated(line_ending)
        .parse(input)
}

fn map(input: &str) -> IResult<&str, Map> {
    take_until(":")
        .precedes(tag(":"))
        .precedes(line_ending)
        .precedes(
            separated_list1(space1.or(line_ending), character::complete::u64).map(|r| {
                let ranges = r
                    .chunks(3)
                    .map(|partial_map| {
                        let dest = partial_map[0];
                        let src = partial_map[1];
                        let len = partial_map[2];
                        Range {
                            lower: src,
                            upper: src + len,
                            offset: (dest as i64) - (src as i64),
                        }
                    })
                    .collect::<Vec<_>>();
                Map::new(ranges)
            }),
        )
        .terminated(line_ending)
        .parse(input)
}

fn parse_data(input: &str) -> IResult<&str, Data> {
    pair(seeds, separated_list1(line_ending, map))
        .map(|(seeds, maps)| Data { seeds, maps })
        .parse(input)
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> u64 {
        let (_, data) = parse_data(input).unwrap();
        data.seeds
            .iter()
            .map(|seed| data.get_seed_location(*seed))
            .min()
            .unwrap()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> u64 {
        let (_, data) = parse_data(input).unwrap();
        data.seeds
            .chunks(2)
            .map(|sp| {
                (sp[0]..sp[0] + sp[1])
                    .into_par_iter()
                    .map(|seed| data.get_seed_location(seed))
                    .min()
                    .unwrap()
            })
            .progress()
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day05/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 35)
    }

    #[test]
    fn test_parse_sample() {
        let (rem, data) = parse_data(SAMPLE).unwrap();
        assert!(rem.is_empty());
        // seed to soil map
        assert_eq!(data.maps[0].get(98), 50);
        assert_eq!(data.maps[0].get(99), 51);
        assert_eq!(data.maps[0].get(50), 52);
        assert_eq!(data.maps[0].get(58), 60);

        // light to temp map
        assert_eq!(data.maps[4].get(46), 82);
    }

    #[test]
    fn parse_seeds() {
        let s = "seeds: 79 14 55 13\n";
        let (rem, v) = seeds(s).unwrap();
        assert!(rem.is_empty());
        assert_eq!(v, vec![79, 14, 55, 13]);
    }

    #[test]
    fn parse_map() {
        let s = r#"seed-to-soil map:
50 98 2
52 50 1
"#;

        let (rem, v) = map(s).unwrap();
        assert!(rem.is_empty());
        let e = Map::new(vec![
            Range {
                lower: 98,
                upper: 100,
                offset: -48,
            },
            Range {
                lower: 50,
                upper: 51,
                offset: 2,
            },
        ]);
        assert_eq!(v, e);
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 323142486)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 46)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 79874951)
    }
}

#[cfg(test)]
mod day05_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
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
