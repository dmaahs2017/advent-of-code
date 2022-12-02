#![feature(test)]
extern crate test;
use aoc_2022::*;

const DAY: u8 = 1;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

/// Convert input &str into an iterator over each elf's total calories
fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split("\n\n").map(|elf| {
        elf.split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .sum()
    })
}

/// Day 1 Puzzle 1
pub mod p1 {
    use super::*;
    /// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    pub fn solve(input: &str) -> usize {
        parse_input(input).max().expect("input was empty")
    }
}

/// day 1 Puzzle 2
pub mod p2 {
    use super::*;
    /// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    pub fn solve(input: &str) -> usize {
        parse_input(input)
            .fold([0, 0, 0], |mut acc, elf_bag: usize| {
                let min = acc.iter_mut().min().unwrap();
                *min = elf_bag.max(*min);
                acc
            })
            .iter()
            .sum()
    }

    #[cfg(test)]
    pub fn solve_with_sorting(input: &str) -> usize {
        let mut v = parse_input(input).collect::<Vec<_>>();
        v.sort();
        v.iter().rev().take(3).sum()
    }
}

#[cfg(test)]
mod day01_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = include_str!("../../inputs/day01/input.txt");
        b.iter(|| p1::solve(input))
    }

    #[bench]
    fn bench_p2_with_sorting(b: &mut Bencher) {
        let input = include_str!("../../inputs/day01/input.txt");
        b.iter(|| p2::solve_with_sorting(input))
    }

    #[test]
    fn p2_works() {
        let input = include_str!("../../inputs/day01/sample.txt");
        assert_eq!(p2::solve(input), 45000)
    }

    #[test]
    fn p2_with_sorting_works() {
        let input = include_str!("../../inputs/day01/sample.txt");
        assert_eq!(p2::solve_with_sorting(input), 45000)
    }

    #[test]
    fn p1_works() {
        let input = include_str!("../../inputs/day01/sample.txt");
        assert_eq!(p1::solve(input), 24000)
    }
}
