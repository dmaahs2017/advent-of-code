#![feature(test)]
extern crate test;
use aoc_2015::*;

const DAY: u8 = 01;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

pub mod p1 {
    pub fn solve(input: &str) -> isize {
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .sum()
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> isize {
        let mut cur = 0;
        let mut sum = 0;
        for c in input.chars() {
            match c {
                '(' => {
                    sum += 1;
                }
                ')' => {
                    sum -= 1;
                }
                _ => {}
            }
            cur += 1;
            if sum < 0 {
                break;
            }
        }
        cur
    }
}

#[cfg(test)]
mod day01_tests {
    use super::*;
    use test::Bencher;

    const SAMPLE: &str = include_str!("../../inputs/day01/sample.txt");

    #[test]
    fn p1_works() {
        assert_eq!(p1::solve("("), 1);
        assert_eq!(p1::solve(")"), -1);
        assert_eq!(p1::solve(SAMPLE), 3)
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2::solve(SAMPLE), 1)
    }

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| p1::solve(SAMPLE))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| p2::solve(SAMPLE))
    }
}
