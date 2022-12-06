#![feature(test)]
extern crate test;
use aoc_2015::*;
use std::collections::HashMap;

const DAY: u8 = 7;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input, "a"),
        p2::solve(input, "a")
    );
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Signal {
    Value(u16),
    Variable(String),
}

impl std::str::FromStr for Signal {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<u16>() {
            Ok(Signal::Value(n))
        } else {
            if s.chars().all(char::is_uppercase) {
                return Err("Expect Signal got operation");
            }
            Ok(Signal::Variable(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Set(Signal),
    And(Signal, Signal),
    Or(Signal, Signal),
    LShift(Signal, Signal),
    RShift(Signal, Signal),
    Not(Signal),
}

fn parse_input(input: &str) -> HashMap<Signal, Operation> {
    input
        .lines()
        .map(|line| {
            let (input, output) = line.split_once(" -> ").expect("line should have an output");
            let output_signal = Signal::Variable(output.to_string());

            let operation = input
                .chars()
                .filter(|c| c.is_uppercase())
                .collect::<String>();
            let op_parts = input.split_whitespace().collect::<Vec<&str>>();
            let input_signal = match operation.as_str() {
                "AND" => Operation::And(
                    op_parts[0].parse().expect("Should be a signal"),
                    op_parts[2].parse().expect("Should be a signal"),
                ),
                "OR" => Operation::Or(
                    op_parts[0].parse().expect("Should be a signal"),
                    op_parts[2].parse().expect("Should be a signal"),
                ),
                "LSHIFT" => Operation::LShift(
                    op_parts[0].parse().expect("Should be a signal"),
                    op_parts[2].parse().expect("Should be a signal"),
                ),
                "RSHIFT" => Operation::RShift(
                    op_parts[0].parse().expect("Should be a signal"),
                    op_parts[2].parse().expect("Should be a signal"),
                ),
                "NOT" => Operation::Not(op_parts[1].parse().expect("Should be a signal")),
                _ => Operation::Set(op_parts[0].parse().expect("Should be a signal")),
            };

            (output_signal, input_signal)
        })
        .collect()
}

fn eval(map: &HashMap<Signal, Operation>, output: &Signal, memo: &mut HashMap<Signal, u16>) -> u16 {
    // memoization
    if let Some(n) = memo.get(output) {
        return *n;
    }
    // base case
    if let Signal::Value(n) = output {
        return *n;
    }

    let v = match map.get(&output).unwrap() {
        Operation::Set(a) => eval(map, a, memo),
        Operation::And(a, b) => {
            let a = eval(map, a, memo);
            let b = eval(map, b, memo);
            a & b
        }
        Operation::Or(a, b) => {
            let a = eval(map, a, memo);
            let b = eval(map, b, memo);
            a | b
        }
        Operation::RShift(a, b) => {
            let a = eval(map, a, memo);
            let b = eval(map, b, memo);
            a >> b
        }
        Operation::LShift(a, b) => {
            let a = eval(map, a, memo);
            let b = eval(map, b, memo);
            a << b
        }
        Operation::Not(a) => {
            let a = eval(map, a, memo);
            !a
        }
    };

    // momoization
    memo.insert(output.clone(), v);
    v
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str, output: &str) -> u16 {
        let map = parse_input(input);
        eval(
            &map,
            &Signal::Variable(output.to_string()),
            &mut Default::default(),
        )
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str, output: &str) -> u16 {
        let mut map = parse_input(input);
        let a = eval(
            &map,
            &Signal::Variable(output.to_string()),
            &mut Default::default(),
        );
        map.insert(
            Signal::Variable("b".to_string()),
            Operation::Set(Signal::Value(a)),
        );
        eval(
            &map,
            &Signal::Variable(output.to_string()),
            &mut Default::default(),
        )
    }
}

#[cfg(test)]
mod day07_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day07/sample.txt");

    #[test]
    fn p1_sample() {
        let expected = vec![
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ];
        for (s, e) in expected {
            assert_eq!(p1::solve(SAMPLE, s), e);
        }
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input, "a"), 16076)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input, "a"), 2797)
    }

    #[test]
    fn parse_input_works() {
        let a = parse_input(SAMPLE);
        dbg!(&a);
        let expected = HashMap::from([
            (
                Signal::Variable('x'.to_string()),
                Operation::Set(Signal::Value(123)),
            ),
            (
                Signal::Variable('y'.to_string()),
                Operation::Set(Signal::Value(456)),
            ),
            (
                Signal::Variable('d'.to_string()),
                Operation::And(
                    Signal::Variable('x'.to_string()),
                    Signal::Variable('y'.to_string()),
                ),
            ),
            (
                Signal::Variable('e'.to_string()),
                Operation::Or(
                    Signal::Variable('x'.to_string()),
                    Signal::Variable('y'.to_string()),
                ),
            ),
            (
                Signal::Variable('f'.to_string()),
                Operation::LShift(Signal::Variable('x'.to_string()), Signal::Value(2)),
            ),
            (
                Signal::Variable('g'.to_string()),
                Operation::RShift(Signal::Variable('y'.to_string()), Signal::Value(2)),
            ),
            (
                Signal::Variable('h'.to_string()),
                Operation::Not(Signal::Variable('x'.to_string())),
            ),
            (
                Signal::Variable('i'.to_string()),
                Operation::Not(Signal::Variable('y'.to_string())),
            ),
        ]);
        assert_eq!(a, expected);
    }
}

#[cfg(test)]
mod day07_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p1::solve(input, "a"))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input, "a"))
    }
}
