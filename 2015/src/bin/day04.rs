#![feature(test)]
extern crate test;
use aoc_2015::*;

const DAY: u8 = 04;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

pub mod p1 {
    use std::process::Command;
    pub fn solve(input: &str) -> usize {
        for i in 0.. {
            let input = format!("{}{}", input.trim(), i);
            let hash = Command::new("md5")
                .args(["-s".to_string(), input])
                .output().unwrap().stdout;
            let hash = String::from_utf8(hash).unwrap();
            if i % 1000 == 0 {
                println!("{}", hash.trim());
            }
            let (_, hash ) = hash.split_once('=').unwrap();
            if hash.trim().chars().take(5).all(|c| c == '0') {
                return i
            }
            
        }
        unreachable!()
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> usize {
        input.len()
    }
}

#[cfg(test)]
mod day04_tests {
    use super::*;
    use test::Bencher;
    
    const SAMPLE: &str = include_str!("../../inputs/day04/sample.txt");

    //#[test]
    //fn p1_works() {
    //    assert_eq!(p1::solve(SAMPLE), 609043)
    //}

    //#[test]
    //fn p2_works() {
    //    assert_eq!(p2::solve(SAMPLE), 12)
    //}


    //#[bench]
    //fn bench_p1(b: &mut Bencher) {
    //    let input = &read_input(DAY);
    //    b.iter(|| p1::solve(input))
    //}

    //#[bench]
    //fn bench_p2(b: &mut Bencher) {
    //    let input = &read_input(DAY);
    //    b.iter(|| p2::solve(input))
    //}
}
