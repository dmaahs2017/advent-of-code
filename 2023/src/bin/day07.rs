#![feature(test)]
extern crate test;
use aoc_2023::*;

use itertools::{Itertools, Position};

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

#[derive(Copy, Clone)]
enum FaceRank {
    Wild = 1,
    Ten = 10,
    Jack,
    Queen,
    King,
    Ace,
}

impl FaceRank {
    fn as_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    bid: u64,
    cards: Vec<u8>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

impl Hand {
    fn new(cards: Vec<u8>, bid: u64) -> Self {
        Self { cards, bid }
    }

    fn jacks_to_wild(&mut self) {
        for c in self
            .cards
            .iter_mut()
            .filter(|c| **c == FaceRank::Jack.as_u8())
        {
            *c = FaceRank::Wild.as_u8();
        }
    }

    fn hand_type(&self) -> HandType {
        use HandType::*;
        let mut map = self.cards.iter().counts();
        let wilds = map.remove(&FaceRank::Wild.as_u8()).unwrap_or_default();

        if wilds == 5 {
            return FiveOfKind;
        }

        let counts = map
            .values()
            .sorted()
            .with_position()
            .map(|(p, n)| match p {
                Position::Last | Position::Only => n + wilds,
                _ => *n,
            })
            .join("");

        match counts.as_str() {
            "5" => FiveOfKind,
            "14" => FourOfKind,
            "23" => FullHouse,
            "113" => ThreeOfKind,
            "122" => TwoPair,
            "1112" => OnePair,
            _ => HighCard,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = Hand> + '_ {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards
            .chars()
            .map(|c| {
                c.to_digit(10).unwrap_or_else(|| match c {
                    'A' => FaceRank::Ace as u32,
                    'K' => FaceRank::King as u32,
                    'Q' => FaceRank::Queen as u32,
                    'J' => FaceRank::Jack as u32,
                    'T' => FaceRank::Ten as u32,
                    _ => unreachable!(),
                }) as u8
            })
            .collect();

        let bid = bid.parse().unwrap();
        Hand::new(cards, bid)
    })
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> u64 {
        parse(input)
            .sorted_by_key(|h| (h.hand_type(), h.cards.clone()))
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid)
            .sum()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> u64 {
        parse(input)
            .map(|mut h| {
                h.jacks_to_wild();
                h
            })
            .sorted_by_key(|h| (h.hand_type(), h.cards.clone()))
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid)
            .sum()
    }
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day07/sample.txt");

    #[test]
    fn test_parse_sample() {
        let hands = parse(SAMPLE).collect::<Vec<_>>();
        let e = vec![
            Hand::new(vec![3, 2, 10, 3, 13], 765),
            Hand::new(vec![10, 5, 5, 11, 5], 684),
            Hand::new(vec![13, 13, 6, 7, 7], 28),
            Hand::new(vec![13, 10, 11, 11, 10], 220),
            Hand::new(vec![12, 12, 12, 11, 14], 483),
        ];
        assert_eq!(hands, e);
    }

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 6440)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 251545216)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 5905)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 250384185)
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
