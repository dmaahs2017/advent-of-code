#![feature(test)]
extern crate test;
use aoc_2023::*;

const DAY: u8 = 2;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

pub fn solve_p1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (id, game) = line.split_once(':').unwrap();
            let id = id.split_once(' ').unwrap().1.parse::<usize>().unwrap();

            let all = game.split([',', ';']).all(|draw| {
                let (n, color) = draw.trim().split_once(' ').unwrap();
                let n = n.parse::<usize>().unwrap();
                match color {
                    "red" => n <= 12,
                    "green" => n <= 13,
                    "blue" => n <= 14,
                    _ => unreachable!("No other color"),
                }
            });

            if all {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (_, game) = line.split_once(':').unwrap();

            let maxes = game.split([',', ';']).fold((0, 0, 0), |mut acc, draw| {
                let (n, color) = draw.trim().split_once(' ').unwrap();
                let n = n.parse::<usize>().unwrap();
                match color {
                    "red" => acc.0 = acc.0.max(n),
                    "green" => acc.1 = acc.1.max(n),
                    "blue" => acc.2 = acc.2.max(n),
                    _ => unreachable!("No other colors should exist"),
                };
                acc
            });
            maxes.0 * maxes.1 * maxes.2
        })
        .sum()
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day02/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 8)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 2176)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 2286)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 63700)
    }
}

#[cfg(test)]
mod day02_benchmarks {
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
