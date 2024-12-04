#![feature(test)]
extern crate test;
use aoc_2024::*;
use regex::Regex;
use nom::{character::complete as character, multi::*, sequence::*, branch::*, IResult, bytes::complete as bytes, Parser};
use nom_supreme::ParserExt;

const DAY: u8 = 3;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, solve_p1(input), solve_p2(input)
    );
}


pub fn solve_p1(input: &str) -> i64 {
    let r = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    r.captures_iter(input).map(|c| c.extract().1).map(|s:[&str; 2]| { 
        let a = s[0].parse::<i64>().unwrap() ;
        let b = s[1].parse::<i64>().unwrap() ;
        a * b
    }).sum()
}

#[derive(Debug)]
enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

fn parse(s: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        bytes::tag("mul(").precedes(separated_pair(character::i64, bytes::tag(","), character::i64)).terminated(bytes::tag(")")).map(|s| Some(Instruction::Mul(s.0, s.1))),
        bytes::tag("don't").map(|_| Some( Instruction::Dont )),
        bytes::tag("do").map(|_| Some( Instruction::Do )),
        bytes::take(1usize).map(|_| None)
    )))(s).map(|x| {
        (x.0, x.1.into_iter().filter_map(|a| a).collect())
    })

}
pub fn solve_p2(input: &str) -> i64 {
    let instrucitons = parse(input).unwrap().1;

    let mut do_flag = true;
    let mut acc = 0;
    for i in instrucitons.iter() {
        match *i {
            Instruction::Dont => do_flag = false,
            Instruction::Do => do_flag = true,
            Instruction::Mul(a, b) => if do_flag {acc += a * b }
        }
    }

    acc

}

#[cfg(test)]
mod day03_tests {
    use super::*;
    
    const SAMPLE: &str = include_str!("../../inputs/day03/sample.txt");
    const SAMPLE2: &str = include_str!("../../inputs/day03/sample2.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 161)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 173731097)
    }

    #[test]
    fn p2_sample() {
        dbg!(SAMPLE2);
        assert_eq!(solve_p2(SAMPLE2), 48)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 93729253)
    }


}

#[cfg(test)]
mod day03_benchmarks {
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
