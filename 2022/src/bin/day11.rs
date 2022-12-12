#![feature(test)]
extern crate test;
use std::{str::FromStr, mem, fmt::Display};

use aoc_2022::*;
use anyhow::{Result, Error, Context, bail};

const DAY: u8 = 11;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

type WorryLevel = usize;

#[derive(Debug)]
enum Op {
    MulOld,
    Add(WorryLevel),
    Mul(WorryLevel),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<WorryLevel>,
    op: Op,
    test: WorryLevel,
    positive_target: usize,
    negative_target: usize,
    inspection_count: usize,
}

impl FromStr for Monkey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let context = || format!(r#"Failed to parse monkey from: "\n{}\n""#, s);
        let mut parts = s.trim().lines().skip(1);

        let item_line = parts.next().with_context(context)?;
        let op_line = parts.next().with_context(context)?;
        let test_line = parts.next().with_context(context)?;
        let pos_target_line = parts.next().with_context(context)?;
        let neg_target_line = parts.next().with_context(context)?;

        let items = item_line.get(18..).with_context(context)?.split(",")
            .map(|item| {
                item.trim().parse::<WorryLevel>()
            }).collect::<Result<_, _>>()?;

        let op_parts = op_line.get(23..).with_context(context)?
            .split_once(" ").with_context(context)?;
        let op = match op_parts.0 {
            "*" => {
                if let Ok(wl) = op_parts.1.parse::<WorryLevel>() {
                    Op::Mul(wl)
                } else {
                    Op::MulOld
                }
            },
            "+" => {
                if let Ok(wl) = op_parts.1.parse::<WorryLevel>() {
                    Op::Add(wl)
                } else {
                    bail!("Add Old is an unsuppored operation");
                }
            },
            _ => bail!("Unsupported operation in {}", s),
        };

        let test = test_line.get(21..).with_context(context)?
            .parse()?;

        let positive_target = pos_target_line.get(29..).with_context(context)?
            .parse()?;

        let negative_target = neg_target_line.get(30..).with_context(context)?
            .parse()?;


        Ok( Self { items, op, test, positive_target, negative_target, inspection_count: 0 })
    }
}

fn parse(input: &str) -> Result<Vec<Monkey>> {
    input.split("\n\n").map(|part| {
        part.parse()
    }).collect()
}

struct KeepAwayGame {
    monkeys: Vec<Monkey>
}

impl Display for KeepAwayGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, m) in self.monkeys.iter().enumerate() {
            let xs = m.items.iter().map(ToString::to_string).collect::<Vec<_>>();
            write!(f, "Monkey {} has Items: {}", i,  xs.join(", "))?;
            writeln!(f)?;
        }
        Ok(())

    }
}

impl Monkey {
    fn throw_items(&mut self) -> Vec<(WorryLevel, usize)> {
        let items = mem::replace(&mut self.items, vec![]);
        self.inspection_count += items.len();
        items.into_iter().map(|mut item| {
            match self.op {
                Op::MulOld => item *= item,
                Op::Add(x) => item += x,
                Op::Mul(x) => item *= x,
            };
            item /= 3;
            let target = if item % self.test == 0 {
                self.positive_target
            } else {
                self.negative_target
            };

            (item, target)
        }).collect()
    }

    fn catch(&mut self, item: WorryLevel) {
        self.items.push(item)
    }
}


impl KeepAwayGame {
    fn new(monkeys: Vec<Monkey>) -> Self {
        Self {monkeys}
    }

    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            let items_and_targets = self.monkeys[idx].throw_items();
            println!("\tMonkey {} throws: ", idx);
            for (item, target) in items_and_targets {
                println!("\t{} -> {}", item, target);
                self.monkeys[target].catch(item);
            }
            println!();

        }
    }

    fn rounds(&mut self, n: usize) {
        println!("Initial State");
        println!("{}", self);
        for i in 0..n {
            self.round();
            println!("After Round {}", i + 1);
            println!("{}", self)
        }
    }

    fn monkey_business(&self) -> usize {
        let mut b = self.monkeys.iter()
            .map(|m| {
                m.inspection_count
            }).collect::<Vec<_>>();
        b.sort();
        b.pop().unwrap() * b.pop().unwrap()
        
    }
}


pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let mut game = KeepAwayGame::new(parse(input).expect("Failed to parse input"));
        dbg!(&game.monkeys[1]);
        game.rounds(20);
        game.monkey_business()
    }
}

pub mod p2 {
    pub fn solve(input: &str) -> usize {
        input.len()
    }
}

#[cfg(test)]
mod day11_tests {
    use super::*;
    
    const SAMPLE: &str = include_str!("../../inputs/day11/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 10605)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 0)
    }

    #[test]
    #[ignore]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 0)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 0)
    }


}

#[cfg(test)]
mod day11_benchmarks {
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
