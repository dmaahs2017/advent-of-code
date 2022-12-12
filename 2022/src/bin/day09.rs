#![feature(test)]
extern crate test;
use std::collections::HashSet;

use aoc_2022::*;

const DAY: u8 = 9;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

struct Simulation<const N_TAILS: usize> {
    head_pos: (isize, isize),
    tails: [(isize, isize); N_TAILS],
    hits: HashSet<(isize, isize)>,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

fn parse(input: &str) -> Vec<(Direction, usize)> {
    input
        .lines()
        .map(|l| {
            let (d, c) = l.split_once(" ").unwrap();
            let d = match d {
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                _ => panic!("Malformed input"),
            };
            let c = c.parse().unwrap();
            (d, c)
        })
        .collect::<Vec<_>>()
}

impl<const N_TAILS: usize> Simulation<N_TAILS> {
    fn new() -> Self {
        let mut s = Self {
            head_pos: (15, 11),
            tails: [(15, 11); N_TAILS],
            hits: HashSet::new(),
        };
        s.hits.insert((15, 11));
        s
    }

    fn show(&self) {
        for i in 0..21 {
            for j in 0..26 {
                if self.head_pos == (i, j) {
                    print!("H ")
                } else if let Some(x) = self.tails.iter().enumerate().find(|x| *x.1 == (i, j)) {
                    print!("{} ", x.0);
                } else {
                    print!(". ")
                }
            }
            println!();
        }
    }

    fn simulate(&mut self, directions: Vec<(Direction, usize)>) {
        for direction in directions {
            println!("{:?}, {}", direction.0, direction.1);
            for _ in 0..direction.1 {
                self.process_direction(direction.0);
                self.show();
                println!();
            }
        }
    }

    fn process_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }

    fn update_tails(&mut self, mut target: (isize, isize)) {
        let mut head = self.head_pos;

        for tail in self.tails.iter_mut() {
            if !is_adjacent(*tail, head) {
                let prev = *tail;
                *tail = target;
                head = *tail;
                target = prev;
            } else {
                break;
            }
        }
        let last_tail = *self.tails.last().unwrap();
        self.hits.insert(last_tail);
    }

    fn move_up(&mut self) {
        let prev_head = self.head_pos;
        self.head_pos.0 -= 1;
        self.update_tails(prev_head);
    }
    fn move_down(&mut self) {
        let prev_head = self.head_pos;
        self.head_pos.0 += 1;
        self.update_tails(prev_head);
    }
    fn move_left(&mut self) {
        let prev_head = self.head_pos;
        self.head_pos.1 -= 1;
        self.update_tails(prev_head);
    }
    fn move_right(&mut self) {
        let prev_head = self.head_pos;
        self.head_pos.1 += 1;
        self.update_tails(prev_head);
    }

    fn count_unique_tail_visits(&self) -> usize {
        self.hits.len()
    }
}

fn is_adjacent(tail_pos: (isize, isize), to: (isize, isize)) -> bool {
    (to.0.saturating_sub(1)..=to.0 + 1)
        .flat_map(|i| std::iter::repeat(i).zip(to.1.saturating_sub(1)..=to.1 + 1))
        .any(|pos| tail_pos == pos)
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let directions = parse(input);
        let mut simulation: Simulation<1> = Simulation::new();
        simulation.simulate(directions);
        simulation.count_unique_tail_visits()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let directions = parse(input);
        let mut simulation: Simulation<9> = Simulation::new();
        simulation.simulate(directions);
        simulation.count_unique_tail_visits()
    }
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day09/sample.txt");
    const SAMPLE_2: &str = include_str!("../../inputs/day09/sample-2.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 13)
    }

    #[test]
    #[ignore]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 6081)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 1)
    }
    #[test]
    fn p2_sample_2() {
        assert_eq!(p2::solve(SAMPLE_2), 36)
    }

    #[test]
    #[ignore]
    fn p2_input() {
        let input = &read_input(DAY);
        assert!(p2::solve(input) < 4794);
        assert_eq!(p2::solve(input), 0);
    }
}

#[cfg(test)]
mod day09_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    #[ignore]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p1::solve(input))
    }

    #[bench]
    #[ignore]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| p2::solve(input))
    }
}
