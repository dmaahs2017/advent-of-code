#![feature(test)]
extern crate test;
use aoc_2015::*;
use std::collections::HashSet;

const DAY: u8 = 3;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

fn houses_visited(input: &str) -> HashSet<(i32, i32)> {
    let mut visted = HashSet::new();
    visted.insert((0, 0));
    input.chars().fold((0, 0), |mut acc, direction| {
        match direction {
            '^' => acc.1 += 1,
            'v' => acc.1 -= 1,
            '>' => acc.0 += 1,
            '<' => acc.0 -= 1,
            _ => {}
        }
        visted.insert(acc);
        acc
    });
    visted
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        houses_visited(input).len()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let santa_route = input.chars().over_evens().collect::<String>();
        let robo_santa_route = input.chars().over_odds().collect::<String>();

        houses_visited(&santa_route)
            .union(&houses_visited(&robo_santa_route))
            .collect::<HashSet<_>>()
            .len()
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;
    use test::Bencher;

    const SAMPLE: &str = include_str!("../../inputs/day03/sample.txt");

    #[test]
    fn p1_works() {
        assert_eq!(p1::solve(SAMPLE), 4)
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2::solve("^v^v^v^v^v"), 11);
        assert_eq!(p2::solve(SAMPLE), 3)
    }

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
