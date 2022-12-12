#![feature(test)]
extern crate test;
use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use aoc_2022::*;

const DAY: u8 = 10;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 is a rendered drawing",
        DAY,
        p1::solve(input)
    );
    p2::solve(input);
}

#[derive(Debug)]
struct Instruction {
    instruction: _Instruction,
    line_number: usize,
}

#[derive(Debug)]
enum _Instruction {
    Noop,
    Add(isize),
}

impl Instruction {
    fn cycles(&self) -> u8 {
        match self.instruction {
            _Instruction::Noop => 1,
            _Instruction::Add(_) => 2,
        }
    }
}

impl FromStr for _Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let context = || format!("Could not parse instruction from: '{}'", s);
        let mut parts = s.split_whitespace();
        match parts.next().with_context(context)? {
            "addx" => Ok(Self::Add(parts.next().with_context(context)?.parse()?)),
            "noop" => Ok(Self::Noop),
            _ => bail!(context()),
        }
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input
        .lines()
        .enumerate()
        .map(|(ln, l)| {
            Ok(Instruction {
                instruction: l.parse::<_Instruction>()?,
                line_number: ln + 1,
            })
        })
        .collect()
}

#[derive(Debug)]
struct Cpu {
    cycle: usize,
    register: isize,
    ip: usize,
    insutruction_cylces_remaining: u8,
    instructions: Vec<Instruction>,
    done: bool,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "                       Cycle: {}", self.cycle)?;
        writeln!(f, "                    Register: {}", self.register)?;
        writeln!(
            f,
            "             Signal Strength: {}",
            self.signal_strength()
        )?;
        writeln!(
            f,
            "                 Instruction: {:?}",
            self.instructions[self.ip].instruction
        )?;
        writeln!(
            f,
            "      Insruction Line Number: {:?}",
            self.instructions[self.ip].line_number
        )?;
        writeln!(
            f,
            "Instruction Cycles Remaining: {}",
            self.insutruction_cylces_remaining
        )?;
        Ok(())
    }
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cycle: 0,
            register: 1,
            ip: 0,
            insutruction_cylces_remaining: instructions[0].cycles(),
            instructions,
            done: false,
        }
    }

    fn advance_ip(&mut self) -> Result<()> {
        if self.ip + 1 >= self.instructions.len() {
            bail!("No remaining instructions")
        }
        self.ip += 1;
        self.insutruction_cylces_remaining = self.instructions[self.ip].cycles();
        Ok(())
    }

    fn tick(&mut self) -> Result<()> {
        if self.done {
            bail!("No more instructions to execute");
        }

        self.cycle += 1;
        self.insutruction_cylces_remaining -= 1;
        if self.insutruction_cylces_remaining > 0 {
            return Ok(());
        }

        match self.instructions[self.ip].instruction {
            _Instruction::Noop => {}
            _Instruction::Add(n) => self.register += n,
        }

        if self.advance_ip().is_err() {
            self.done = true;
        }
        Ok(())
    }

    fn tick_n(&mut self, ticks: usize) -> Result<()> {
        for _ in 0..ticks {
            self.tick()
                .with_context(|| format!("Tick {} times", ticks))?;
        }
        println!("{}", self);
        Ok(())
    }

    fn signal_strength(&self) -> isize {
        self.register * self.cycle as isize
    }
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> isize {
        let instructions = parse(input).unwrap();
        let mut cpu = Cpu::new(instructions);
        cpu.tick_n(20).unwrap();
        (0..5).fold(cpu.signal_strength(), |acc, _| {
            cpu.tick_n(40).unwrap();
            acc + cpu.signal_strength()
        })
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> String {
        let instructions = parse(input).unwrap();
        let mut cpu = Cpu::new(instructions);
        let mut display = String::new();

        for _ in 0..6 {
            for col in 0..40 {
                let sprite_pos = cpu.register;

                if col == sprite_pos || col == sprite_pos - 1 || col == sprite_pos + 1 {
                    display.push('#');
                } else {
                    display.push('.');
                }
                cpu.tick().unwrap();
            }
            display.push('\n');
        }
        display
    }
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day10/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 13360)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 15260)
    }

    #[test]
    fn p2_sample() {
        println!("{}", p2::solve(SAMPLE));
        assert_eq!(
            p2::solve(SAMPLE),
            include_str!("../../inputs/day10/expected-p2-sample.txt")
        );
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        println!("{}", p2::solve(input));
        assert_eq!(
            p2::solve(input),
            include_str!("../../inputs/day10/expected-p2-input.txt")
        );
    }
}

#[cfg(test)]
mod day10_benchmarks {
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
