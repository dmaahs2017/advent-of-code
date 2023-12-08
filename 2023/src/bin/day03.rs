#![feature(test)]
extern crate test;
use aoc_2023::*;

use std::collections::{HashMap, HashSet};

use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::{is_not, take_till1},
    character::complete::digit1,
    combinator::iterator,
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

const DAY: u8 = 3;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Symbol(SpanIVec2<'a>),
    Number(SpanIVec2<'a>),
}

fn with_xy(span: Span) -> SpanIVec2 {
    span.map_extra(|_| {
        IVec2::new(
            span.get_column() as i32 - 1,
            span.location_line() as i32 - 1,
        )
    })
}
fn parse_grid(input: Span) -> IResult<Span, Vec<Token>> {
    let mut it = iterator(
        input,
        alt((
            digit1.map(with_xy).map(Token::Number).map(Some),
            is_not(".\n123456789")
                .map(with_xy)
                .map(Token::Symbol)
                .map(Some),
            take_till1(|c: char| c.is_ascii_digit() || c != '.' && c != '\n').map(|_| None),
        )),
    );

    let parsed = it.flatten().collect::<Vec<_>>();
    let res = it.finish();
    res.map(|(input, _)| (input, parsed))
}

pub fn solve_p1(input: &str) -> u32 {
    let tokens = parse_grid(Span::new(input)).unwrap().1;

    let symbol_map = tokens
        .iter()
        .filter_map(|token| match token {
            Token::Symbol(s) => Some(s.extra),
            _ => None,
        })
        .collect::<HashSet<_>>();

    tokens
        .iter()
        .filter_map(|token| {
            let Token::Number(num) = token else {
                return None;
            };

            let num_len = num.fragment().len() as i32;

            [
                // east border
                IVec2::new(num_len, 0),
                // west border
                IVec2::new(-1, 0),
            ]
            .into_iter()
            .chain(
                //north border
                (-1..=num_len).map(|x| IVec2::new(x, 1)),
            )
            .chain(
                // south border
                (-1..=num_len).map(|x| IVec2::new(x, -1)),
            )
            .map(|offset| offset + num.extra)
            .any(|pos| symbol_map.contains(&pos))
            .then(|| num.parse::<u32>().unwrap())
        })
        .sum()
}

pub fn solve_p2(input: &str) -> usize {
    let tokens = parse_grid(Span::new(input)).unwrap().1;

    let number_map = tokens
        .iter()
        .filter_map(|t| match t {
            Token::Number(n) => Some(n),
            _ => None,
        })
        .flat_map(|num| {
            (num.extra.x..num.extra.x + num.len() as i32)
                .map(move |x| (IVec2::new(x, num.extra.y), num))
        })
        .collect::<HashMap<_, _>>();

    tokens
        .iter()
        .filter_map(|token| {
            let Token::Symbol(sym) = token else {
                return None;
            };

            let gears = [
                IVec2::new(-1, 0),
                IVec2::new(1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 1),
                IVec2::new(1, 1),
                IVec2::new(1, -1),
                IVec2::new(-1, 1),
                IVec2::new(-1, -1),
            ]
            .into_iter()
            .map(|offset| offset + sym.extra)
            .filter_map(|pos| number_map.get(&pos))
            .collect::<HashSet<_>>();

            if gears.len() == 2 {
                Some(
                    gears
                        .iter()
                        .map(|g| g.parse::<usize>().unwrap())
                        .product::<usize>(),
                )
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day03/sample.txt");
    const SAMPLE_2: &str = include_str!("../../inputs/day03/edge_case_1.txt");
    const SAMPLE_3: &str = include_str!("../../inputs/day03/edge_case_2.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 4361)
    }

    #[test]
    fn p1_edge() {
        assert_eq!(solve_p1(SAMPLE_2), 0)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 530495)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 467835)
    }

    #[test]
    fn p2_edge() {
        assert_eq!(solve_p2(SAMPLE_3), 81)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 80253814)
    }
}

#[cfg(test)]
mod day03_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p1(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
