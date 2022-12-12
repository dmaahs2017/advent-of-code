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


type Item = usize;

#[derive(Debug)]
enum Op {
    MulOld,
    Add(Item),
    Mul(Item),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<Item>,
    op: Op,
    test: Item,
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
                item.trim().parse::<Item>()
            }).collect::<Result<_, _>>()?;

        let op_parts = op_line.get(23..).with_context(context)?
            .split_once(" ").with_context(context)?;
        let op = match op_parts.0 {
            "*" => {
                if let Ok(wl) = op_parts.1.parse::<Item>() {
                    Op::Mul(wl)
                } else {
                    Op::MulOld
                }
            },
            "+" => {
                if let Ok(wl) = op_parts.1.parse::<Item>() {
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
    monkeys: Vec<Monkey>,
    one_ring: usize,
    relief: bool,
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
    fn throw_items(&mut self, relief: bool) -> Vec<(Item, usize)> {
        let items = mem::replace(&mut self.items, vec![]);
        self.inspection_count += items.len();
        items.into_iter().map(|mut item| {
            // apply the monkey's operation to the worry level
            match &self.op {
                Op::MulOld => item = item.pow(2),
                Op::Add(x) => item += x,
                Op::Mul(x) => item *= x,
            };

            // apply relief to the worry level (if part 1)
            if relief {
                item /= 3
            }

            // select target monkey based on the modulo test
            let target = if item % self.test == 0 {
                self.positive_target
            } else {
                self.negative_target
            };

            // return tuple of item and target
            (item, target)
        }).collect()
    }

    fn catch(&mut self, item: Item) {
        self.items.push(item)
    }
}


impl KeepAwayGame {
    fn new(monkeys: Vec<Monkey>, relief: bool) -> Self {
        let one_ring = monkeys.iter().map(|m| m.test).reduce(|a, b| a * b).expect("Should have > 1 monkey");
        Self {monkeys, relief, one_ring}
    }

    fn round(&mut self) {
        for idx in 0..self.monkeys.len() {
            let items_and_targets = self.monkeys[idx].throw_items(self.relief);
            for (mut item, target) in items_and_targets {
                item %= self.one_ring;
                self.monkeys[target].catch(item);
            }

        }
    }

    fn rounds(&mut self, n: usize) {
        for _ in 0..n {
            self.round();
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
        let mut game = KeepAwayGame::new(parse(input).expect("Failed to parse input"), true);
        game.rounds(20);
        game.monkey_business()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let mut game = KeepAwayGame::new(parse(input).expect("Failed to parse input"), false);
        game.rounds(10000);
        game.monkey_business()
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
        assert_eq!(p1::solve(input), 66124)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 2713310158)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 19309892877)
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
