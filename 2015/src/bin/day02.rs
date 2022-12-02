#![feature(test)]
extern crate test;
use aoc_2015::*;

const DAY: u8 = 02;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

fn parse_input(input: &str) -> impl Iterator<Item=(usize, usize, usize)> + '_ {
    input.lines().map(|line| {
        let lens = line
            .split("x")
            .map(|n| { n.parse::<usize>().expect("Failed to parse side length") })
            .collect::<Vec<_>>();
        (lens[0], lens[1], lens[2])
    })
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        parse_input(input).map(|(l, w, h)| {
            let sides = [l*w, l*h, w*h];
            2 * sides.iter().sum::<usize>() + sides.iter().min().unwrap()
        }).sum()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        parse_input(input).map(|(l, w, h)| {
            let half_perims = [l + w, l + h, w + h];
            2 * half_perims.iter().min().unwrap() + l * w * h
        }).sum()
    }
}

#[cfg(test)]
mod day02_tests {
    use super::*;
    use test::Bencher;
    
    const SAMPLE: &str = include_str!("../../inputs/day02/sample.txt");

    #[test]
    fn p1_works() {
        assert_eq!(p1::solve(SAMPLE), 58 + 43)
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2::solve(SAMPLE), 34 + 14)
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
