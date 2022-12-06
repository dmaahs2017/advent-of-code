#![feature(test, pattern)]
extern crate test;
use aoc_2015::*;
use std::str::pattern::*;

const DAY: u8 = 8;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

struct EscapeSeqSearcher<'a> {
    s: &'a str,
    idx: usize,
}

unsafe impl<'a> Searcher<'a> for EscapeSeqSearcher<'a> {
    fn haystack(&self) -> &'a str {
        self.s
    }

    fn next(&mut self) -> SearchStep {
        if self.idx >= self.s.len() {
            return SearchStep::Done;
        }

        let bytes = self.s.as_bytes();
        if self.s.as_bytes()[self.idx] == b'\\' {
            match bytes[self.idx + 1] {
                b'\\' | b'"' => {
                    let i = self.idx;
                    self.idx += 2;
                    return SearchStep::Match(i, i + 2);
                }
                b'x' => {
                    let i = self.idx;
                    self.idx += 4;
                    return SearchStep::Match(i, i + 4);
                }
                _ => unreachable!(),
            }
        } else {
            let i = self.idx;
            self.idx += 1;
            return SearchStep::Reject(i, i);
        }
    }
}

struct EscapeSeqPattern;
impl<'a> Pattern<'a> for EscapeSeqPattern {
    type Searcher = EscapeSeqSearcher<'a>;
    fn into_searcher(self, haystack: &'a str) -> Self::Searcher {
        Self::Searcher {
            s: haystack,
            idx: 0,
        }
    }
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let literal_chars = line.len();
                let in_mem = line[1..line.len() - 1]
                    .matches(EscapeSeqPattern)
                    .fold(literal_chars - 2, |acc, m| acc - (m.len() - 1));

                literal_chars - in_mem
            })
            .sum()
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> usize {
        input.len()
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day08/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 12)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 1350)
    }

    #[test]
    #[ignore]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 0)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 0)
    }
}

#[cfg(test)]
mod day08_benchmarks {
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
