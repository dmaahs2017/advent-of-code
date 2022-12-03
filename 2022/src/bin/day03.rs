#![feature(test)]
extern crate test;
use aoc_2022::*;
use std::collections::HashSet;

const DAY: u8 = 03;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

fn score(c: char) -> usize {
    let v = match c {
        'A'..='Z' => c as u8 - b'A' + 27,
        'a'..='z' => c as u8 - b'a' + 1,
        _ => 0,
    };
    v as usize
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let (c_1, c_2) = line.split_at(line.len() / 2);
                let set = c_1.chars().collect::<HashSet<_>>();
                let dup = c_2
                    .chars()
                    .find(|c| set.contains(c))
                    .expect("Why didn't this have a duplicate???");
                score(dup)
            })
            .sum()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let lines = input.lines().collect::<Vec<_>>();
        lines
            .chunks(3)
            .map(|group| {
                let final_set = group
                    .iter()
                    .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
                    .reduce(|mut a, b| {
                        intersect(&mut a, &b);
                        a
                    })
                    .expect("I should not have an empty iterator");

                let badge = final_set
                    .into_iter()
                    .next()
                    .expect("I should have just 1 value in this set");
                score(badge)
            })
            .sum()
    }

    pub fn solve_with_default_intersection(input: &str) -> usize {
        let lines = input.lines().collect::<Vec<_>>();
        lines
            .chunks(3)
            .map(|group| {
                let final_set = group
                    .iter()
                    .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
                    .reduce(|a, b| a.intersection(&b).cloned().collect::<HashSet<_>>())
                    .expect("I should not have an empty iterator");

                let badge = final_set
                    .into_iter()
                    .next()
                    .expect("I should have just 1 value in this set");
                score(badge)
            })
            .sum()
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;
    use test::Bencher;

    const SAMPLE: &str = include_str!("../../inputs/day03/sample.txt");

    #[test]
    fn p1_works() {
        assert_eq!(p1::solve(SAMPLE), 157)
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2::solve(SAMPLE), 70)
    }

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p1::solve(input))
    }

    #[bench]
    fn bench_p2_in_place_intersection(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input))
    }

    #[bench]
    fn bench_p2_std_intersection(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve_with_default_intersection(input))
    }
}
