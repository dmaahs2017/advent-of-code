#![feature(test)]
extern crate test;
use anyhow::{Context, Result};
use aoc_2022::*;

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

type Stack = Vec<char>;
type Instruction = (u32, usize, usize);

fn parse(s: &str) -> Result<(Vec<Stack>, Vec<Instruction>)> {
    let begin_instructions_idx = s
        .find("move")
        .with_context(|| "Failed to find the begining of the instructions")?;

    let instructions_slice = &s[begin_instructions_idx..];
    let instructions = instructions_slice
        .lines()
        .map(|line| -> Result<_> {
            let v = line.split_whitespace().collect::<Vec<_>>();
            Ok((
                v[1].parse::<u32>()?,
                v[3].parse::<usize>()? - 1,
                v[5].parse::<usize>()? - 1,
            ))
        })
        .collect::<Result<_, _>>()?;

    let stacks_slice = &s[..begin_instructions_idx - 1];
    let stack_lines = stacks_slice.lines().collect::<Vec<_>>();

    let n_stacks = stacks_slice[stacks_slice.len() - 3..stacks_slice.len() - 2].parse()?;
    let mut stacks = vec![vec![]; n_stacks];

    for &row in stack_lines[..stack_lines.len() - 1].iter().rev() {
        for (stack_idx, slice_idx) in (1..row.len()).step_by(4).enumerate() {
            let c = row.as_bytes()[slice_idx] as char;
            if c.is_alphabetic() {
                stacks[stack_idx].push(c)
            }
        }
    }

    Ok((stacks, instructions))
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> String {
        let (mut stacks, instructions) = parse(input).expect("Malformed input");

        for instruction in instructions {
            for _ in 0..instruction.0 {
                let m = stacks[instruction.1]
                    .pop()
                    .expect("There should be a box in the input stack to move");
                stacks[instruction.2].push(m);
            }
        }

        stacks.into_iter().filter_map(|mut c| c.pop()).collect()
    }
}

pub mod p2 {
    use super::*;

    pub fn solve(input: &str) -> String {
        let (mut stacks, instructions) = parse(input).expect("Malformed input");
        for instruction in instructions {
            let mut ms = (0..instruction.0)
                .map(|_| {
                    stacks[instruction.1]
                        .pop()
                        .expect("There should be a box to move of the stack")
                })
                .collect::<Vec<_>>();
            stacks[instruction.2].extend(ms.drain(..).rev())
        }

        stacks.into_iter().filter_map(|mut c| c.pop()).collect()
    }
}

#[cfg(test)]
mod day05_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day05/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), String::from("CMZ"))
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), String::from("RLFNRTNFB"))
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), String::from("MCD"))
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), String::from("MHQTLJRLB"))
    }
}

#[cfg(test)]
mod day05_benchmarks {
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
