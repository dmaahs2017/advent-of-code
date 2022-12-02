#![feature(test)]
extern crate test;
use aoc_2015::*;

const DAY: u8 = 05;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

pub mod p1 {
    pub fn solve(input: &str) -> usize {
        input.lines().filter(|line| {
            let vowel_count = line.chars().filter(|c| {
                match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => true,
                    _ => false,
                }
            }).count();
            let has_duplicate_in_row = line.as_bytes().windows(2).filter(|window| window[0] == window[1]).count() > 0;
            let has_naughty_string = line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy");
            
            vowel_count >= 3 && has_duplicate_in_row && !has_naughty_string
        }).count()
    }
}

pub mod p2 {
    use std::collections::HashMap;
    pub fn solve(input: &str) -> usize {
        input.lines().filter(|line| {
            let staggered_repetition = line.as_bytes().windows(3).filter(|window| window[0] == window[1]).count() > 0;

            let mut map: HashMap<String, (usize, usize)> = HashMap::new();
            //let line = line.chars().collect::<Vec<_>>();
            for (start_index, pair) in line.as_bytes().windows(2).enumerate() {
                let pair = String::from_utf8(pair.to_vec()).unwrap();
                if let Some((end_index, cnt)) = map.get_mut(&pair) {
                    if start_index + 1 != *end_index {
                        *cnt += 1;
                    }
                } else {
                    map.insert(pair, ( start_index + 1, 1 ));
                }

            }
            dbg!(&map.get("qj"));
            let has_two_pair = map.into_values().any(|e| e.1 >= 2);

            
            staggered_repetition && has_two_pair
        }).count()
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;
    use test::Bencher;
    
    const SAMPLE: &str = include_str!("../../inputs/day05/sample.txt");

    #[test]
    fn p1_works() {
        assert_eq!(p1::solve(SAMPLE), 2)
    }

    #[test]
    fn p2_works() {
        assert_eq!(p2::solve("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(p2::solve("uurcxstgmygtbstg"), 0);
        assert_eq!(p2::solve("ieodomkazucvgmuy"), 0);
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
