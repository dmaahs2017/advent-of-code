#![feature(test)]
extern crate test;
use aoc_2022::*;
use anyhow::{Result, bail, Context};

const DAY: u8 = 12;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

const START: Height = 0;
const END: Height = 27;
type Position = (usize, usize);
type Height = u8;
fn parse(input: &str) -> Result<(Position, Position, Vec<Vec<Height>>)> {
    let grid: Vec<Vec<u8>> = input.lines()
        .map(|l| {
            l.chars().map(|c| {
                match c {
                    'a'..='z' => Ok(c as u8 - b'a' + 1),
                    'S' => Ok(START),
                    'E' => Ok(END),
                    _ => bail!("Unrecognized height mapping: {}", c),
                }
                
            }).collect()
        }).collect::<Result<_, _>>()?;

    let start = grid.iter()
        .enumerate()
        .find_map(|(x, row)| {
            if let Some(y) = row.iter().position(|c| *c == START) {
                Some((x, y))
            } else {
                None
            }
        }).with_context(|| "Grid should have a start")?;

    let end = grid.iter()
        .enumerate()
        .find_map(|(x, row)| {
            if let Some(y) = row.iter().position(|c| *c == END) {
                Some((x, y))
            } else {
                None
            }
        }).with_context(|| "Grid should have an end")?;

    Ok((start, end, grid))
}


pub mod p1 {
    use std::collections::HashSet;

    use super::*;
    pub fn solve(input: &str) -> usize {
        let (start, end, grid) = parse(input).expect("Failed to parse input");
        solve_rec(start, end, &grid, &mut Default::default())
    }

    fn solve_rec(start: Position, end: Position, grid: &Vec<Vec<u8>>, visited: &mut HashSet<Position>) -> usize {
        if visited.contains(&start) {
            return 0
        }
        if start == end {
            return 0
        }
        let sh = grid[start.0][start.1];
        visited.insert(start);

        let adjacent = (start.0.saturating_sub(1)..=start.0 + 1)
            .zip(start.1.saturating_sub(1)..=start.1 + 1)
            .filter(|p| *p != start);

        let mut min = usize::MAX;
        for (x, y) in adjacent {
            dbg!(x, y);
            if let Some(row) = grid.get(x) {
                if let Some(h) = row.get(y) {
                    dbg!(h, sh);
                    if (*h as i8 - sh as i8).abs() < 2 {
                        min = min.min( 1 + solve_rec((x, y), end, grid, visited) )
                    }
                }
            }
        }
        visited.remove(&start);
        return min;
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> usize {
        input.len()
    }
}

#[cfg(test)]
mod day12_tests {
    use super::*;
    
    const SAMPLE: &str = include_str!("../../inputs/day12/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 31)
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
mod day12_benchmarks {
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
