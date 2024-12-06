#![feature(test)]
extern crate test;
use aoc_2024::*;
use glam::IVec2;

use std::collections::HashSet;
use std::default::Default;

use rayon::prelude::*;

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

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
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
    Empty,
    Blocked(IVec2),
    Guard(IVec2),
}

#[derive(Debug, Clone, Default)]
struct Board {
    starting_pos: IVec2,
    blocked_squares: HashSet<IVec2>,
    guard: (IVec2, Direction),
    guard_log: HashSet<(IVec2, Direction)>,
    distinct_locations: HashSet<IVec2>,
}

impl Board {
    #[allow(dead_code)]
    fn pretty_print(&self, height: i32, width: i32) {
        for y in 0..height {
            println!();
            for x in 0..width {
                let pos = IVec2::new(x, y);

                if pos == self.starting_pos {
                    print!("^");
                } else if let Some(g) = self.guard_log.iter().find(|lg| lg.0 == pos) {
                    match g.1 {
                        Direction::North => print!("|"),
                        Direction::South => print!("|"),
                        Direction::East => print!("-"),
                        Direction::West => print!("-"),
                    }
                } else if self.blocked_squares.contains(&pos) {
                    print!("#")
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    fn tick(&mut self) {
        self.guard_log.insert(self.guard);
        self.distinct_locations.insert(self.guard.0);
        if self.blocked_squares.contains(&self.guard_facing_pos()) {
            self.turn_guard_right();
        } else {
            self.move_guard_forward();
        }
    }

    fn has_visited_current_pos(&self) -> bool {
        self.guard_log.contains(&self.guard)
    }

    fn is_guard_in_bounds(&self, height: i32, width: i32) -> bool {
        self.guard.0.x >= 0
            && self.guard.0.x < width
            && self.guard.0.y >= 0
            && self.guard.0.y < height
    }

    fn move_guard_forward(&mut self) {
        self.guard.0 = self.guard_facing_pos();
    }

    fn turn_guard_right(&mut self) {
        self.guard.1 = self.guard.1.turn_right()
    }

    fn guard_facing_pos(&self) -> IVec2 {
        match self.guard.1 {
            Direction::North => self.guard.0 + IVec2::new(0, -1),
            Direction::East => self.guard.0 + IVec2::new(1, 0),
            Direction::South => self.guard.0 + IVec2::new(0, 1),
            Direction::West => self.guard.0 + IVec2::new(-1, 0),
        }
    }
}

fn parse_square(s: Span) -> IResult<Span, Square> {
    let p = IVec2::new(s.get_column() as i32 - 1, s.location_line() as i32 - 1);

    alt((
        value(Square::Empty, bytes::tag(".")),
        value(Square::Blocked(p), bytes::tag("#")),
        value(Square::Guard(p), bytes::tag("^")),
    ))(s)
}

fn parse_board(s: Span) -> IResult<Span, Board> {
    let (s, xs) = separated_list1(character::line_ending, many1(parse_square))(s)?;

    let mut board = Board::default();
    for s in xs.into_iter().flatten() {
        match s {
            Square::Empty => (),
            Square::Blocked(p) => {
                board.blocked_squares.insert(p);
            }
            Square::Guard(p) => board.guard.0 = p,
        }
    }
    board.starting_pos = board.guard.0;

    Ok((s, board))
}

pub fn solve_p1(input: &str) -> usize {
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let mut board = parse_board(Span::new(input)).unwrap().1;

    while board.is_guard_in_bounds(height, width) {
        board.tick();
    }
    board.distinct_locations.len()
}

pub fn solve_p2(input: &str) -> usize {
    let width = input.lines().count() as i32;
    let height = input.lines().next().unwrap().len() as i32;
    let starting_board = parse_board(Span::new(input)).unwrap().1;

    let mut board = starting_board.clone();
    while board.is_guard_in_bounds(height, width) {
        board.tick();
    }

    board
        .distinct_locations
        .par_iter()
        .filter(|&new_block| {
            if *new_block == starting_board.guard.0 {
                return false;
            }

            let mut test_board = starting_board.clone();
            test_board.blocked_squares.insert(*new_block);

            while test_board.is_guard_in_bounds(height, width) {
                test_board.tick();
                if test_board.has_visited_current_pos() {
                    return true;
                }
            }

            false
        })
        .count()
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
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 6)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 1946)
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
