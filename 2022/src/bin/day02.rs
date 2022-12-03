#![feature(test)]
extern crate test;
use aoc_2022::*;

const DAY: u8 = 2;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY, p1::solve(input), p2::solve(input)
    );
}

/// Parse input into an iterator over meaningful data
fn parse_input(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
    input.split('\n').filter(|s| s.len() >= 3).map(|round| {
        let round = round.as_bytes();
        let them = round[0] as char;
        let me = round[2] as char;
        (them, me)
    })
}

/// Score a round of rock paper scisors
/// 1 = Rock, 2 = Paper, 3 = Scisors
fn score(them: i8, me: i8) -> usize {
    let mut score = me as usize;
    match me - them {
        1 | -2 => score += 6,
        0 => score += 3,
        -1 | 2 => {}
        _ => panic!("Invalid input was given"),
    }
    score
}

pub mod p1 {
    use super::*;
    /// What would your total score be if everything goes exactly according to your strategy guide?
    pub fn solve(input: &str) -> usize {
        parse_input(input)
            .map(|(them, me)| {
                let them = them as i8 - 'A' as i8 + 1;
                let me = me as i8 - 'W' as i8;
                score(them, me)
            })
            .sum()
    }
}

pub mod p2 {
    use super::*;
    /// Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
    pub fn solve(input: &str) -> usize {
        parse_input(input)
            .map(|(them, should_win)| {
                let them = them as i8 - 'A' as i8 + 1;
                let me = match should_win {
                    'X' => ( them - 1 + 3 ) % 3,  // I should lose
                    'Y' => them,                  // Tie
                    'Z' => ( them + 1 ) % 3,      // I should win
                    _ => unreachable!("Should win field can only be Win, Lose, or Draw"),
                };
                score(them, me)
            })
            .sum()
    }
}

#[cfg(test)]
mod day02_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = include_str!("../../inputs/day02/input.txt");
        b.iter(|| p1::solve(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = include_str!("../../inputs/day02/input.txt");
        b.iter(|| p2::solve(input))
    }

    #[test]
    fn p2_works() {
        let input = include_str!("../../inputs/day02/sample.txt");
        assert_eq!(p2::solve(input), 12)
    }

    #[test]
    fn p1_works() {
        let input = include_str!("../../inputs/day02/sample.txt");
        assert_eq!(p1::solve(input), 15)
    }
}
