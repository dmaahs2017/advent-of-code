#![feature(test)]
extern crate test;
use aoc_2022::*;
use regex::Regex;


const DAY: u8 = 15;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

#[derive(Debug, Copy, Clone)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self { Self { row, col } }

    fn distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }
}

/// Returns a list of Points and their manhattan distance to the nearest beacon
fn parse(input: &str) -> Vec<(Point, usize)> {
    let re = Regex::new(r"(-?\d+)").expect("Invalid regex");
    input.lines()
        .map(|line| {
            let values = re.captures_iter(line).map(|x| {
                x[0].parse().unwrap()
            }).collect::<Vec<_>>();
            let c1 = values[0];
            let r1 = values[1];

            let c2 = values[2];
            let r2 = values[3];

            let p1 = Point::new(r1, c1);
            let p2 = Point::new(r2, c2);
            let distance = p1.distance(&p2);
            (p1, distance)
        }).collect()
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let sensors = parse(input);
        let row = 10;
        (isize::MIN..=isize::MAX).filter(|col| {
            let p = Point::new(row, *col);
            sensors.iter().any(|sensor| p.distance(&sensor.0) <= sensor.1)
        }).count()
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> usize {
        input.len()
    }
}

#[cfg(test)]
mod day15_tests {
    use super::*;
    
    const SAMPLE: &str = include_str!("../../inputs/day15/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 26)
    }

    #[test]
    #[ignore]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 0)
    }

    #[test]
    #[ignore]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 0)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 0)
    }


}

#[cfg(test)]
mod day15_benchmarks {
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
