#![feature(test)]
extern crate test;
use aoc_2023::*;

const DAY: u8 = 1;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

/// Concat the first and last digit of each line, parse the number and sum them all
pub fn solve_p1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut iter = line.chars().filter_map(|c| char::to_digit(c, 10));

            let first = iter.next().unwrap();
            if let Some(last) = iter.next_back() {
                first * 10 + last
            } else {
                first * 10 + first
            }
        })
        .sum::<u32>() as usize
}

const DIGIT_WORDS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

/// Concat the first and last digit, or spelled digit, of each line, parse the number and sum them all
pub fn solve_p2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = (0..DIGIT_WORDS.len())
                .filter_map(|dwi| line.find(DIGIT_WORDS[dwi]).map(|i| (i, dwi % 9)))
                .min_by_key(|fws| fws.0)
                .unwrap()
                .1
                + 1;

            let last = (0..DIGIT_WORDS.len())
                .filter_map(|dwi| line.rfind(DIGIT_WORDS[dwi]).map(|i| (i, dwi % 9)))
                .max_by_key(|fws| fws.0)
                .unwrap()
                .1
                + 1;

            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day01/sample.txt");
    const SAMPLE_2: &str = include_str!("../../inputs/day01/sample_2.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 142)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 56506)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE_2), 281)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 56017)
    }
}

#[cfg(test)]
mod day01_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p1(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
