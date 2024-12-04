#![feature(test)]
extern crate test;
use aoc_2024::*;

const DAY: u8 = 4;

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
    let mut acc = 0;

    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            // if i >= 3 then can look up
            // if j >= 3 then can look left
            // if i <= grid.len() - 4 then can look down
            // if j <= grid[0].len() - 4 then can look right

            // look up
            if i >= 3 {
                let s: String = (0..4).map(|offset| grid[i - offset][j] as char).collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look left
            if j >= 3 {
                let s: String = (0..4).map(|offset| grid[i][j - offset] as char).collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look down
            if i <= grid.len() - 4 {
                let s: String = (0..4).map(|offset| grid[i + offset][j] as char).collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look right
            if j <= grid[0].len() - 4 {
                let s: String = (0..4).map(|offset| grid[i][j + offset] as char).collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look up_right
            if i >= 3 && j <= grid[0].len() - 4 {
                let s: String = (0..4)
                    .map(|offset| grid[i - offset][j + offset] as char)
                    .collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look up_left
            if i >= 3 && j >= 3 {
                let s: String = (0..4)
                    .map(|offset| grid[i - offset][j - offset] as char)
                    .collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look down_right
            if i <= grid.len() - 4 && j <= grid[0].len() - 4 {
                let s: String = (0..4)
                    .map(|offset| grid[i + offset][j + offset] as char)
                    .collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
            // look down_left
            if i <= grid.len() - 4 && j >= 3 {
                let s: String = (0..4)
                    .map(|offset| grid[i + offset][j - offset] as char)
                    .collect();
                if s == "XMAS" {
                    acc += 1;
                }
            }
        }
    }
    acc
}

pub fn solve_p2(input: &str) -> usize {
    let mut acc = 0;

    let grid = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    for i in 0..grid.len() {
        if i == 0 || i == grid.len() - 1 {
            continue;
        }
        for j in 0..grid[0].len() {
            if grid[i][j] != b'A' || j == 0 || j == grid[0].len() - 1 {
                continue;
            }

            #[rustfmt::skip]
            // negative diagonal
            let a: String = [grid[i-1][j-1] as char, grid[i][j] as char, grid[i+1][j+1] as char].into_iter().collect();
            #[rustfmt::skip]
            let a2: String = [grid[i-1][j-1] as char, grid[i][j] as char, grid[i+1][j+1] as char].into_iter().rev().collect();
            // positive diagonal
            #[rustfmt::skip]
            let b: String = [grid[i+1][j-1] as char, grid[i][j] as char, grid[i-1][j+1] as char].into_iter().collect();
            #[rustfmt::skip]
            let b2: String = [grid[i+1][j-1] as char, grid[i][j] as char, grid[i-1][j+1] as char].into_iter().rev().collect();

            if (a == "MAS" || a2 == "MAS") && (b == "MAS" || b2 == "MAS") {
                acc += 1
            }
        }
    }
    acc
}

#[cfg(test)]
mod day04_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day04/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 18)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 2562)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 9)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 1902)
    }
}

#[cfg(test)]
mod day04_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    #[ignore]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p1(input))
    }

    #[bench]
    #[ignore]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
