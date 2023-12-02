#![feature(test)]
extern crate test;
use aoc_2023::*;

const DAY: u8 = 2;

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
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter_map(|line| {
                let (id, game) = line.split_once(':').unwrap();
                let id = id.split_once(' ').unwrap().1.parse::<usize>().unwrap();

                let rounds = game
                    .split(';')
                    .map(|round| {
                        round
                            .split(',')
                            .filter_map(|n_cubes| n_cubes.trim().split_once(' '))
                            .fold((0, 0, 0), |mut acc, n_cube| {
                                let n = n_cube.0.parse::<usize>().unwrap();
                                match n_cube.1 {
                                    "red" => acc.0 += n,
                                    "green" => acc.1 += n,
                                    "blue" => acc.2 += n,
                                    _ => unreachable!("Should be no other colors"),
                                };
                                acc
                            })
                    })
                    .collect::<Vec<_>>();

                if rounds
                    .iter()
                    .all(|round| round.0 <= 12 && round.1 <= 13 && round.2 <= 14)
                {
                    Some(id)
                } else {
                    None
                }
            })
            .sum()
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> usize {
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
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day02/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 8)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 2176)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 2286)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 63700)
    }
}

#[cfg(test)]
mod day02_benchmarks {
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
