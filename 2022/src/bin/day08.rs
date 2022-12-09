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
    if row == height - 1 || col == width - 1 {
        return true;
    }

    let up = 0..row;
    let down = row + 1..height;
    let left = 0..col;
    let right = col + 1..width;

    !up.zip(iter::repeat(col))
        .any(|(x, y)| hm[x][y] >= max_tree_height)
        || !down
            .zip(iter::repeat(col))
            .any(|(x, y)| hm[x][y] >= max_tree_height)
        || !right
            .zip(iter::repeat(row))
            .any(|(y, x)| hm[x][y] >= max_tree_height)
        || !left
            .zip(iter::repeat(row))
            .any(|(y, x)| hm[x][y] >= max_tree_height)
}

fn scenic_score(row: usize, col: usize, hm: &HeightMap) -> usize {
    let up = up_visibility(row, col, hm);
    let down = down_visibility(row, col, hm);
    let left = left_visibility(row, col, hm);
    let right = right_visibility(row, col, hm);

    up * down * left * right
}

fn up_visibility(row: usize, col: usize, hm: &HeightMap) -> usize {
    let max_height = hm[row][col];
    if row == 0 {
        return 0;
    }

    let mut went_to_end = true;
    let mut count = (0..row)
        .rev()
        .zip(iter::repeat(col))
        .map(|(x, y)| hm[x][y])
        .take_while(|h| {
            went_to_end = *h < max_height;
            *h < max_height
        })
        .count();
    if !went_to_end {
        count += 1;
    }
    count
}

fn down_visibility(row: usize, col: usize, hm: &HeightMap) -> usize {
    let max_height = hm[row][col];
    if row == hm.len() - 1 {
        return 0;
    }

    let mut went_to_end = true;
    let mut count = (row + 1..hm.len())
        .map(|x| hm[x][col])
        .take_while(|h| {
            went_to_end = *h < max_height;
            *h < max_height
        })
        .count();
    if !went_to_end {
        count += 1;
    }
    count
}

fn left_visibility(row: usize, col: usize, hm: &HeightMap) -> usize {
    let max_height = hm[row][col];
    if col == 0 {
        return 0;
    }

    let mut went_to_end = true;
    let mut count = (0..col)
        .rev()
        .zip(iter::repeat(row))
        .map(|(y, x)| hm[x][y])
        .take_while(|h| {
            went_to_end = *h < max_height;
            *h < max_height
        })
        .count();
    if !went_to_end {
        count += 1;
    }
    count
}

fn right_visibility(row: usize, col: usize, hm: &HeightMap) -> usize {
    let max_height = hm[row][col];
    if col == hm[0].len() - 1 {
        return 0;
    }

    let mut went_to_end = true;
    let mut count = (col + 1..hm[0].len())
        .zip(iter::repeat(row))
        .map(|(y, x)| hm[x][y])
        .take_while(|h| {
            went_to_end = *h < max_height;
            *h < max_height
        })
        .count();

    if !went_to_end {
        count += 1;
    }
    count
}

fn parse(input: &str) -> HeightMap {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

pub mod p1 {
    use super::*;

    pub fn solve(input: &str) -> usize {
        let hm = parse(input);
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
                score = scenic_score(row, col, &hm).max(score);
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
    fn up_visibility_works() {
        let hm = parse(SAMPLE);
        assert_eq!(up_visibility(2, 2, &hm), 1);
        assert_eq!(up_visibility(1, 2, &hm), 1);
        assert_eq!(up_visibility(0, 2, &hm), 0);
        assert_eq!(up_visibility(1, 1, &hm), 1);
        assert_eq!(up_visibility(2, 0, &hm), 2);
        assert_eq!(up_visibility(4, 3, &hm), 4);
        assert_eq!(up_visibility(3, 2, &hm), 2);
    }

    #[test]
    fn down_visibility_works() {
        let hm = parse(SAMPLE);
        assert_eq!(down_visibility(2, 2, &hm), 1);
        assert_eq!(down_visibility(1, 2, &hm), 2);
        assert_eq!(down_visibility(0, 2, &hm), 1);
        assert_eq!(down_visibility(4, 0, &hm), 0);
        assert_eq!(down_visibility(3, 4, &hm), 1);
    }

    #[test]
    fn left_visibility_works() {
        let hm = parse(SAMPLE);
        assert_eq!(left_visibility(2, 2, &hm), 1);
        assert_eq!(left_visibility(1, 2, &hm), 1);
        assert_eq!(left_visibility(0, 2, &hm), 2);
        assert_eq!(left_visibility(4, 0, &hm), 0);
        assert_eq!(left_visibility(0, 3, &hm), 3);
    }

    #[test]
    fn right_visibility_works() {
        let hm = parse(SAMPLE);
        assert_eq!(right_visibility(2, 2, &hm), 1);
        assert_eq!(right_visibility(1, 2, &hm), 2);
        assert_eq!(right_visibility(0, 2, &hm), 1);
        assert_eq!(right_visibility(4, 0, &hm), 1);
        assert_eq!(right_visibility(0, 4, &hm), 0);
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 199272)
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
    #[ignore]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input))
    }
}
