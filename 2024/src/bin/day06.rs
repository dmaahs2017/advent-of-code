#![feature(test)]
extern crate test;
use aoc_2024::*;
use glam::IVec2;

use std::collections::HashSet;
use std::default::Default;

use nom::{
    branch::*, bytes::complete as bytes, character::complete as character, combinator::value,
    multi::*, IResult,
};
use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;

const DAY: u8 = 6;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Square {
    Empty(IVec2),
    Blocked(IVec2),
    Guard(IVec2),
}

#[derive(Debug, Default)]
struct Board {
    empty_squares: Vec<IVec2>,
    blocked_squares: Vec<IVec2>,
    guard: IVec2,
    guard_direction: Direction,
    guard_visited: HashSet<IVec2>,
}

impl Board {
    fn advance_guard(&mut self) {
        while self.blocked_squares.contains(&self.guard_facing_pos()) {
            self.guard_direction = self.guard_direction.turn_right();
        }
        self.guard_visited.insert(self.guard);
        self.guard = self.guard_facing_pos();
    }

    fn guard_facing_pos(&self) -> IVec2 {
        match self.guard_direction {
            Direction::North => self.guard + IVec2::new(0, -1),
            Direction::East => self.guard + IVec2::new(1, 0),
            Direction::South => self.guard + IVec2::new(0, 1),
            Direction::West => self.guard + IVec2::new(-1, 0),
        }
    }
}

fn parse_square(s: Span) -> IResult<Span, Square> {
    let p = IVec2::new(s.get_column() as i32 - 1, s.location_line() as i32 - 1);

    alt((
        value(Square::Empty(p), bytes::tag(".")),
        value(Square::Blocked(p), bytes::tag("#")),
        value(Square::Guard(p), bytes::tag("^")),
    ))(s)
}

fn parse_board(s: Span) -> IResult<Span, Board> {
    let (s, xs) = separated_list1(character::line_ending, many1(parse_square))(s)?;

    let mut board = Board::default();

    for s in xs.into_iter().flatten() {
        match s {
            Square::Empty(p) => board.empty_squares.push(p),
            Square::Blocked(p) => board.blocked_squares.push(p),
            Square::Guard(p) => board.guard = p,
        }
    }

    Ok((s, board))
}

pub fn solve_p1(input: &str) -> usize {
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let mut board = parse_board(Span::new(input)).unwrap().1;
    while board.guard.x >= 0
        && board.guard.x < width
        && board.guard.y >= 0
        && board.guard.y < height
    {
        board.advance_guard();
    }
    board.guard_visited.len()
}

pub fn solve_p2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day06/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 41)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_ne!(solve_p1(input), 5443, "Answer is too low");
        assert_eq!(solve_p1(input), 5444);
    }

    #[test]
    #[ignore]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 0)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 0)
    }
}

#[cfg(test)]
mod day06_benchmarks {
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
