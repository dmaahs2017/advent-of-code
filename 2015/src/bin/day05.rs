#![feature(test, array_windows)]
extern crate test;
use aoc_2015::*;

const DAY: u8 = 5;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

pub mod p1 {
    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter(|line| {
                let vowel_count = line
                    .chars()
                    .filter(|c| match c {
                        'a' | 'e' | 'i' | 'o' | 'u' => true,
                        _ => false,
                    })
                    .count();
                let has_duplicate_in_row = line
                    .as_bytes()
                    .windows(2)
                    .filter(|window| window[0] == window[1])
                    .count()
                    > 0;
                let has_naughty_string = line.contains("ab")
                    || line.contains("cd")
                    || line.contains("pq")
                    || line.contains("xy");

                vowel_count >= 3 && has_duplicate_in_row && !has_naughty_string
            })
            .count()
    }
}

pub mod p2 {
    use std::collections::HashMap;

    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .filter(|line| has_staggered_repitition(line) && has_non_overlapping_pair(line))
            .count()
    }

    fn has_non_overlapping_pair(line: &str) -> bool {
        line.as_bytes()
            .array_windows::<2>()
            .enumerate()
            .fold(
                HashMap::new(),
                |mut map: HashMap<[u8; 2], Vec<usize>>, (idx, pair)| {
                    map.entry(*pair).or_default().push(idx);
                    map
                },
            )
            .into_values()
            .map(|v| v.array_windows::<2>().map(|&[a, b]| b - a).sum::<usize>())
            .filter(|sum| *sum > 1)
            .count()
            > 0
    }

    fn has_staggered_repitition(line: &str) -> bool {
        line.as_bytes()
            .array_windows::<3>()
            .filter(|window| window[0] == window[2])
            .count()
            > 0
    }

    #[test]
    #[cfg(test)]
    fn has_non_overlapping_pair_works() {
        let one = "xyaaabc"; // false
        let two = "xabcyyab"; // true: ab
        let three = "uurcxstgmygtbstg"; // true 'st'
        assert!(!has_non_overlapping_pair(one));
        assert!(has_non_overlapping_pair(two));
        assert!(has_non_overlapping_pair(three));
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;
    use test::Bencher;

    const SAMPLE: &str = include_str!("../../inputs/day05/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 2)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 255)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(p2::solve("uurcxstgmygtbstg"), 0);
        assert_eq!(p2::solve("ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 55)
    }

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
