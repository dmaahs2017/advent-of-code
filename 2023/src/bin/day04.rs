#![feature(test)]
extern crate test;
use aoc_2023::*;

use std::collections::HashSet;

const DAY: u8 = 4;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

#[derive(Debug)]
struct Card<'a> {
    winning_values: HashSet<&'a str>,
    values: HashSet<&'a str>,
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let (_, a) = line.split_once(':').unwrap();

            let (values, winners) = a.split_once('|').unwrap();
            Card {
                values: values.split_ascii_whitespace().collect(),
                winning_values: winners.split_ascii_whitespace().collect(),
            }
        })
        .collect()
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let cards = parse_cards(input);
        cards.iter().map(score).sum()
    }

    fn score(card: &Card) -> usize {
        let n_winners = card.values.intersection(&card.winning_values).count() as u32;
        2usize.pow(n_winners) >> 1
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let scores = parse_cards(input).iter().map(score).collect::<Vec<_>>();
        let mut counts = vec![1; scores.len()];

        for i in 0..scores.len() {
            for awarded_index in i + 1..=scores[i] + i {
                counts[awarded_index] += counts[i];
            }
        }

        counts.iter().sum()
    }

    fn score(card: &Card) -> usize {
        card.values.intersection(&card.winning_values).count()
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day04/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 13)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 32001)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 30)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 5037841)
    }
}

#[cfg(test)]
mod day04_benchmarks {
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
