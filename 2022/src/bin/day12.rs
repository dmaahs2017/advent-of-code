#![feature(test)]
extern crate test;
use anyhow::{bail, Context, Result};
use aoc_2022::*;
use pathfinding::prelude::astar;

const DAY: u8 = 12;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

const START: u8 = 0;
const LOW_LEVEL: u8 = 1;
const END: u8 = 27;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn distance(&self, other: &Self) -> usize {
        self.row.abs_diff(other.row) + self.col.abs_diff(other.col)
    }

    fn successors(&self, grid: &[Vec<u8>]) -> Vec<(Position, usize)> {
        let max_height = grid.len() - 1;
        let max_width = grid[0].len() - 1;
        let item = grid[self.row][self.col];
        let mut successors = vec![];

        // down
        if self.row > 0 && grid[self.row - 1][self.col] <= item + 1 {
            successors.push((Position::new(self.row - 1, self.col), 1));
        }
        // up
        if self.row < max_height && grid[self.row + 1][self.col] <= item + 1 {
            successors.push((Position::new(self.row + 1, self.col), 1));
        }
        // left
        if self.col > 0 && grid[self.row][self.col - 1] <= item + 1 {
            successors.push((Position::new(self.row, self.col - 1), 1));
        }
        // right
        if self.col < max_width && grid[self.row][self.col + 1] <= item + 1 {
            successors.push((Position::new(self.row, self.col + 1), 1));
        }

        successors
    }
}

fn parse(input: &str) -> Result<Vec<Vec<u8>>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'a'..='z' => Ok(c as u8 - b'a' + 1),
                    'S' => Ok(START),
                    'E' => Ok(END),
                    _ => bail!("Unrecognized height mapping: {}", c),
                })
                .collect()
        })
        .collect()
}

fn get_start(grid: &[Vec<u8>]) -> Result<Position> {
    grid.iter()
        .enumerate()
        .find_map(|(x, row)| {
            if let Some(y) = row.iter().position(|c| *c == START) {
                Some(Position::new(x, y))
            } else {
                None
            }
        })
        .with_context(|| "Grid should have a start")
}

fn get_all_starts(grid: &[Vec<u8>]) -> Vec<Position> {
    grid.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(y, &c)| {
                    if c == START || c == LOW_LEVEL {
                        Some(Position::new(x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn get_end(grid: &[Vec<u8>]) -> Result<Position> {
    grid.iter()
        .enumerate()
        .find_map(|(x, row)| {
            if let Some(y) = row.iter().position(|c| *c == END) {
                Some(Position::new(x, y))
            } else {
                None
            }
        })
        .with_context(|| "Grid should have an end")
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let grid = parse(input).expect("Failed to parse input");
        let start = get_start(&grid).unwrap();
        let end = get_end(&grid).unwrap();

        astar(
            &start,
            |p| p.successors(&grid),
            |p| p.distance(&end),
            |p| p == &end,
        )
        .expect("Path not found")
        .1
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let grid = parse(input).expect("Failed to parse input");
        let end = get_end(&grid).unwrap();
        let starts = get_all_starts(&grid);

        starts
            .into_iter()
            .filter_map(|start| {
                astar(
                    &start,
                    |p| p.successors(&grid),
                    |p| p.distance(&end),
                    |p| p == &end,
                )
                .map(|p| p.1)
            })
            .min()
            .unwrap()
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
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 437)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 29)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 430)
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
