#![feature(test)]
extern crate test;
use std::iter;

use aoc_2022::*;

const DAY: u8 = 8;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

type HeightMap = Vec<Vec<u32>>;
fn is_edge_visible(row: usize, col: usize, hm: &HeightMap) -> bool {
    let max_tree_height = hm[row][col];
    let height = hm.len();
    let width = hm[0].len();

    if row == 0 || col == 0 {
        return true;
    }
    if row == height - 1 || col == width-1 {
        return true;
    }

    let up = 0..row;
    let down = row+1..height;
    let left = 0..col;
    let right = col+1..width;

    !up
        .zip(iter::repeat(col))
        .any(|(x, y)| hm[x][y] >= max_tree_height)
    || 

    !down
        .zip(iter::repeat(col))
        .any(|(x, y)| hm[x][y] >= max_tree_height)
    || 

    !right
        .zip(iter::repeat(row))
        .any(|(y, x)| hm[x][y] >= max_tree_height)

    || 

    !left
        .zip(iter::repeat(row))
        .any(|(y, x)| hm[x][y] >= max_tree_height)
}

fn scenic_score(row: usize, col: usize, hm: &HeightMap) -> usize {
    let max_tree_height = hm[row][col];
    let height = hm.len();
    let width = hm[0].len();


    let up = 0..row;
    let down = row+1..height;
    let left = 0..col;
    let right = col+1..width;

    (
        up
        .zip(iter::repeat(col))
        .position(|(x, y)| hm[x][y] >= max_tree_height).unwrap_or(row)
    )
        *
    (
        down
        .zip(iter::repeat(col))
        .position(|(x, y)| hm[x][y] >= max_tree_height).unwrap_or(height - row)
    )
    *


    (
        left
        .zip(iter::repeat(row))
        .position(|(y, x)| hm[x][y] >= max_tree_height).unwrap_or(col)
    )
    *


    (
        right
        .zip(iter::repeat(row))
        .position(|(y, x)| hm[x][y] >= max_tree_height).unwrap_or(width - col)
    )
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let hm: HeightMap = input
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();

        let mut count = 0;
        for row in 0..hm.len() {
            for col in 0..hm[0].len() {
                if is_edge_visible(row, col, &hm) {
                    count += 1;
                }
            }
        }
        count
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let hm: HeightMap = input
            .lines()
            .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect();

        let mut score = 0;
        for row in 0..hm.len() {
            for col in 0..hm[0].len() {
                score = scenic_score(row, col, &hm).max(score)
            }
        }
        score
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day08/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 21)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 1794)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 8)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 0)
    }
}

#[cfg(test)]
mod day08_benchmarks {
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
