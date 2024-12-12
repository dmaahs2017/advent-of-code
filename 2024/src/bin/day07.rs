#![feature(test)]
extern crate test;
use aoc_2024::*;
use itertools::Itertools;
use nom::{
    bytes::complete as bytes, character::complete as character, multi::*, sequence::*, IResult,
    Parser,
};
use tracing::info;

const DAY: u8 = 7;
const OPERATIONS: [char; 2] = ['+', '*'];
const OPERATIONS_2: [char; 3] = ['+', '*', '|'];

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

#[derive(Debug)]
struct RopeBridge {
    total: i64,
    operands: Vec<i64>,
}

fn parse_rope_bridge(s: &str) -> IResult<&str, RopeBridge> {
    separated_pair(
        character::i64,
        bytes::tag(": "),
        separated_list1(bytes::tag(" "), character::i64),
    )
    .map(|pair| RopeBridge {
        total: pair.0,
        operands: pair.1,
    })
    .parse(s)
}
fn parse(s: &str) -> IResult<&str, Vec<RopeBridge>> {
    separated_list1(character::line_ending, parse_rope_bridge)(s)
}

#[tracing::instrument(ret)]
fn test_bridge(bridge: &RopeBridge) -> bool {
    let n_operators = bridge.operands.len() - 1;
    tracing::info!(n_operators);
    (0..n_operators)
        .map(|_| OPERATIONS)
        .multi_cartesian_product()
        .any(|mut operators| {
            info!("{:?}", operators);
            bridge
                .operands
                .iter()
                .copied()
                .reduce(|acc, right| {
                    match operators
                        .pop()
                        .expect("Always have correct amount of operators")
                    {
                        '*' => acc * right,
                        '+' => acc + right,
                        _ => unreachable!("No other operators supported"),
                    }
                })
                .expect("Should always have at least 1 operand")
                == bridge.total
        })
}

pub fn solve_p1(input: &str) -> i64 {
    let bridges = parse(input).unwrap().1;
    bridges
        .into_iter()
        .filter(test_bridge)
        .map(|b| b.total)
        .sum()
}

#[tracing::instrument(ret)]
fn test_bridge_2(bridge: &RopeBridge) -> bool {
    let n_operators = bridge.operands.len() - 1;
    tracing::info!(n_operators);
    (0..n_operators)
        .map(|_| OPERATIONS_2)
        .multi_cartesian_product()
        .any(|mut operators| {
            info!("{:?}", operators);
            bridge
                .operands
                .iter()
                .copied()
                .reduce(|acc, right| {
                    match operators
                        .pop()
                        .expect("Always have correct amount of operators")
                    {
                        '*' => acc * right,
                        '+' => acc + right,
                        '|' => format!("{}{}", acc, right)
                            .parse::<i64>()
                            .expect("two +ints should always concat to a +int"),
                        _ => unreachable!("No other operators supported"),
                    }
                })
                .expect("Should always have at least 1 operand")
                == bridge.total
        })
}

pub fn solve_p2(input: &str) -> i64 {
    let bridges = parse(input).unwrap().1;
    bridges
        .into_iter()
        .filter(test_bridge_2)
        .map(|b| b.total)
        .sum()
}

#[cfg(test)]
mod day07_tests {
    use super::*;
    use rstest::rstest;

    const SAMPLE: &str = include_str!("../../inputs/day07/sample.txt");

    #[rstest]
    #[case("190: 10 19", true)]
    #[case("3267: 81 40 27", true)]
    #[case("83: 17 5", false)]
    #[case("156: 15 6", false)]
    #[case("7290: 6 8 6 15", false)]
    #[case("161011: 16 10 13", false)]
    #[case("192: 17 8 14", false)]
    #[case("21037: 9 7 18 13", false)]
    #[case("292: 11 6 16 20", true)]
    fn test_bridge_1(#[case] input: &str, #[case] ans: bool) {
        let b = parse_rope_bridge(input).unwrap().1;
        assert_eq!(test_bridge(&b), ans)
    }

    #[test_log::test]
    fn test_bridge_tracing() {
        let b = parse_rope_bridge("292: 11 6 16 20").unwrap().1;
        assert!(test_bridge(&b))
    }

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 3749)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 12940396350192)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 11387)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 106016735664498)
    }
}

#[cfg(test)]
mod day07_benchmarks {
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
