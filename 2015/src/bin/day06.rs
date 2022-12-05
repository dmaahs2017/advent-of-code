#![feature(test)]
extern crate test;
use aoc_2015::*;

const DAY: u8 = 6;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

struct Point {
    x: usize,
    y: usize,
}
enum Op {
    On,
    Off,
    Toggle,
}
fn parse_input(input: &str) -> impl Iterator<Item = (Point, Point, Op)> + '_ {
    input.lines().map(|line| {
        let words = line.split_whitespace().collect::<Vec<_>>();

        let op = if words[0] == "toggle" {
            Op::Toggle
        } else if words[1] == "on" {
            Op::On
        } else {
            Op::Off
        };

        let (a, b) = if words[0] == "toggle" {
            (
                words[1].split_once(',').expect("Should have comma"),
                words[3].split_once(',').expect("Should have comma"),
            )
        } else {
            (
                words[2].split_once(',').expect("Should have comma"),
                words[4].split_once(',').expect("Should have comma"),
            )
        };

        (
            Point {
                x: a.0.parse::<usize>().unwrap(),
                y: a.1.parse::<usize>().unwrap(),
            },
            Point {
                x: b.0.parse::<usize>().unwrap(),
                y: b.1.parse::<usize>().unwrap(),
            },
            op,
        )
    })
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let mut canvas = vec![vec![false; 1000]; 1000];

        for (p1, p2, op) in parse_input(input) {
            for i in p1.x..=p2.x {
                for j in p1.y..=p2.y {
                    match op {
                        Op::On => canvas[i][j] = true,
                        Op::Off => canvas[i][j] = false,
                        Op::Toggle => canvas[i][j] = !canvas[i][j],
                    }
                }
            }
        }

        canvas.into_iter().flatten().filter(|b| *b).count()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let mut canvas = vec![vec![0usize; 1000]; 1000];

        for (p1, p2, op) in parse_input(input) {
            for i in p1.x..=p2.x {
                for j in p1.y..=p2.y {
                    match op {
                        Op::On => canvas[i][j] += 1,
                        Op::Off => canvas[i][j] = canvas[i][j].saturating_sub(1),
                        Op::Toggle => canvas[i][j] += 2,
                    }
                }
            }
        }

        canvas.into_iter().flatten().sum()
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day06/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 2)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 569999)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 5)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 17836115)
    }
}

#[cfg(test)]
mod day06_benchmarks {
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
