#![feature(test)]
extern crate test;
use aoc_2023::*;

use std::cmp::*;
use std::collections::*;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
    multi::separated_list1,
    sequence::separated_pair,
    *,
};
use nom_supreme::ParserExt;

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

type Rank = u8;

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    bid: u64,
    cards: Vec<Rank>,
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
    fn new(cards: Vec<Rank>, bid: u64) -> Self {
        Self { cards, bid }
    }
    fn hand_type(&self) -> HandType {
        let map = self
            .cards
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Rank, u8>, c| {
                *acc.entry(*c).or_default() += 1u8;
                acc
            });

        if *map.values().max().unwrap() == 5 {
            HandType::FiveOfKind
        } else if *map.values().max().unwrap() == 4 {
            HandType::FourOfKind
        } else if *map.values().max().unwrap() == 3 && *map.values().min().unwrap() == 2 {
            HandType::FullHouse
        } else if *map.values().max().unwrap() == 3 {
            HandType::ThreeOfKind
        } else if map.values().filter(|v| **v == 2).count() == 2 {
            HandType::TwoPair
        } else if *map.values().max().unwrap() == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct JokerHand {
    cards: Vec<u8>,
    bid: u64,
}

impl JokerHand {
    fn new(cards: Vec<u8>, bid: u64) -> Self {
        JokerHand { cards, bid }
    }

    fn hand_type(&self) -> HandType {
        use HandType::*;
        let mut map = self
            .cards
            .iter()
            .fold(HashMap::new(), |mut acc: HashMap<Rank, u8>, c| {
                *acc.entry(*c).or_default() += 1u8;
                acc
            });

        dbg!(self);
        let wild_count = map.remove(&1).unwrap_or_default();
        if wild_count == 5 || *map.values().max().unwrap() + wild_count == 5 {
            FiveOfKind
        } else if *map.values().max().unwrap() + wild_count == 4 {
            FourOfKind
        } else if *map.values().max().unwrap() + wild_count == 3
            && *map.values().min().unwrap() == 2
            || *map.values().max().unwrap() == 3 && *map.values().min().unwrap() + wild_count == 2
        {
            FullHouse
        } else if *map.values().max().unwrap() + wild_count == 3 {
            ThreeOfKind
        } else if map.values().filter(|v| **v == 2).count() == 2 {
            if wild_count == 1 {
                FullHouse
            } else {
                TwoPair
            }
        } else if *map.values().max().unwrap() + wild_count == 2 {
            OnePair
        } else {
            HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &JokerHand) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &JokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    separated_list1(
        line_ending,
        separated_pair(
            take_until(" ").map(|s: &str| {
                s.chars()
                    .map(|c| {
                        c.to_digit(10).unwrap_or_else(|| match c {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 11,
                            'T' => 10,
                            _ => unreachable!(),
                        })
                    })
                    .map(|d| d as u8)
                    .collect()
            }),
            tag(" "),
            character::complete::u64,
        )
        .map(|(hand, bid)| Hand::new(hand, bid)),
    )
    .terminated(line_ending)
    .parse(input)
}

fn parse_wild(input: &str) -> IResult<&str, Vec<JokerHand>> {
    separated_list1(
        line_ending,
        separated_pair(
            take_until(" ").map(|s: &str| {
                s.chars()
                    .map(|c| {
                        c.to_digit(10).unwrap_or_else(|| match c {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 1,
                            'T' => 10,
                            _ => unreachable!(),
                        })
                    })
                    .map(|d| d as u8)
                    .collect()
            }),
            tag(" "),
            character::complete::u64,
        )
        .map(|(hand, bid)| JokerHand::new(hand, bid)),
    )
    .terminated(line_ending)
    .parse(input)
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> u64 {
        let (_, mut hands) = parse(input).unwrap();
        hands.sort();

        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1) * hand.bid)
            .sum()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> u64 {
        let (_, mut hands) = parse_wild(input).unwrap();

        for hand in hands.iter() {
            println!("{:?}: {:?}", hand, hand.hand_type());
        }

        hands.sort();
        hands
            .iter()
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
    fn test_ordering() {
        let mut hands = vec![
            Hand::new(vec![3, 2, 10, 3, 13], 0),
            Hand::new(vec![10, 5, 5, 11, 5], 0),
            Hand::new(vec![13, 13, 6, 7, 7], 0),
            Hand::new(vec![13, 10, 11, 11, 10], 0),
            Hand::new(vec![12, 12, 12, 11, 14], 0),
        ];
        hands.sort_by(|a, b| b.cmp(a));

        let e = vec![
            Hand::new(vec![12, 12, 12, 11, 14], 0), // 3 kind,
            Hand::new(vec![10, 5, 5, 11, 5], 0),    // 3 kind
            Hand::new(vec![13, 13, 6, 7, 7], 0),    // 2 pair
            Hand::new(vec![13, 10, 11, 11, 10], 0), // 2 pair
            Hand::new(vec![3, 2, 10, 3, 13], 0),    // high card
        ];

        assert_eq!(hands, e);
    }

    #[test]
    fn test_parse_sample() {
        let (rem, hands) = parse(SAMPLE).unwrap();
        let e = vec![
            Hand::new(vec![3, 2, 10, 3, 13], 765),
            Hand::new(vec![10, 5, 5, 11, 5], 684),
            Hand::new(vec![13, 13, 6, 7, 7], 28),
            Hand::new(vec![13, 10, 11, 11, 10], 220),
            Hand::new(vec![12, 12, 12, 11, 14], 483),
        ];
        assert!(rem.is_empty());
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
