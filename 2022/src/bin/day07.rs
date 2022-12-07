#![feature(test, absolute_path)]
extern crate test;
use std::{collections::HashMap, path::PathBuf};

use anyhow::{bail, Context, Result};
use aoc_2022::*;

const DAY: u8 = 7;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

fn parse(input: &str) -> Result<HashMap<PathBuf, usize>> {
    input
        .lines()
        .filter(|l| !l.starts_with("dir") && !l.starts_with("$ ls"))
        .try_fold(
            (HashMap::new(), PathBuf::new()),
            |(mut map, mut cwd), line| {
                if line.starts_with("$ cd") {
                    let new_dir = line.get(5..).with_context(|| {
                        format!("cd command has no target directory. Line: \"{}\"", line)
                    })?;

                    if new_dir.trim().is_empty() {
                        bail!("cd command target should not be blank. Line: \"{}\"", line)
                    }

                    if new_dir == ".." {
                        cwd.pop();
                    } else {
                        cwd = cwd.join(new_dir);
                    }
                } else {
                    let bytes = line
                        .split_once(" ")
                        .with_context(|| {
                            format!(
                                "File entry should have bytes and file name. Line: \"{}\"",
                                line
                            )
                        })?
                        .0
                        .parse::<usize>()
                        .with_context(|| {
                            format!("Bytes should be an integer. Line: \"{}\"", line)
                        })?;

                    for ancestor in cwd.ancestors() {
                        *map.entry(ancestor.to_path_buf()).or_default() += bytes;
                    }
                }
                Ok((map, cwd))
            },
        )
        .map(|x| x.0)
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let map = parse(input).expect("Failed to parse input");
        map.values()
            .filter_map(|&bytes| if bytes <= 100_000 { Some(bytes) } else { None })
            .sum()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        const NEEDED_SPACE: usize = 30_000_000;
        let map = parse(input).expect("Failed to parse input");
        let used_space = map
            .get(&PathBuf::from(r"/"))
            .expect("Root dir should exist");

        let must_free = used_space - NEEDED_SPACE;

        map.values()
            .cloned()
            .filter(|bytes| *bytes >= must_free)
            .min()
            .expect("An answer should exist")
    }
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day07/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 95437)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 1642503)
    }

    #[test]
    #[ignore]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 24933642)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 0)
    }
}

#[cfg(test)]
mod day07_benchmarks {
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
