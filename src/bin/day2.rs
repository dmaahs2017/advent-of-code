#![feature(test)]
extern crate test;

fn main() {
    let input = include_str!("../../inputs/day2/input.txt");
    println!("Part 1 answer = {}", p1::solve(input));
    println!("Part 2 answer = {}", p2::solve(input));
}

#[repr(i8)]
#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scisors = 3,
}

impl From<char> for Shape {
    fn from(c: char) -> Shape {
        match c {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scisors,
            _ => unreachable!("Input data has no other values"),
        }
    }
}

/// Convert input &str an iter of scores
fn parse_input(input: &str) -> impl Iterator<Item = (Shape, Shape)> + '_ {
    input.split('\n').filter(|s| s.len() >= 3).map(|round| {
        let round = round.as_bytes();
        let them = round[0] as char;
        let me = round[2] as char;
        (them.into(), me.into())
    })
}

/// Day 1 Puzzle 1
pub mod p1 {
    use super::*;
    /// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
    pub fn solve(input: &str) -> usize {
        parse_input(input)
            .map(|(them, me)| {
                let mut score = me as usize;
                match me as i8 - them as i8 {
                    1 | -2 => score += 6,
                    0 => score += 3,
                    -1 | 2 => {}
                    _ => unreachable!("This should not be a possible score!"),
                }
                score
            })
            .sum()
    }
}

/// day 1 Puzzle 2
pub mod p2 {
    use super::*;
    /// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
    pub fn solve(input: &str) -> usize {
        todo!("Solve part 2")
    }
}

#[cfg(test)]
mod day2_tests {
    use super::*;

    #[test]
    fn p1_works() {
        let input = include_str!("../../inputs/day2/sample.txt");
        assert_eq!(p1::solve(input), 15)
    }
}
