#![feature(test, array_windows)]
extern crate test;
use std::collections::HashSet;

use aoc_2022::*;

const DAY: u8 = 14;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        p1::solve(input),
        p2::solve(input)
    );
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }
    fn down_left(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }
    fn down_right(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }

    fn falls_on_line(&self, line: &[Point]) -> bool {
        line.array_windows::<2>().any(|[startpoint, endpoint]| {
            if startpoint.x == endpoint.x {
                let ymin = startpoint.y.min(endpoint.y);
                let ymax = startpoint.y.max(endpoint.y);
                self.x == startpoint.x && ymin <= self.y && self.y <= ymax
            } else if startpoint.y == endpoint.y {
                let xmin = startpoint.x.min(endpoint.x);
                let xmax = startpoint.x.max(endpoint.x);
                self.y == startpoint.y && xmin <= self.x && self.x <= xmax
            } else {
                panic!("Lines should always be either horizontal or vertical")
            }
        })
    }
}

#[derive(Debug)]
struct Game {
    obstacles: Vec<Vec<Point>>,
    sand: HashSet<Point>,
    start_point: Point,
    lowest_obstacle: usize,
}

impl Game {
    fn new(obstacles: Vec<Vec<Point>>, floor: bool) -> Self {
        let mut lowest_obstacle = obstacles
            .iter()
            .map(|line| {
                line.iter()
                    .map(|p| p.y)
                    .max()
                    .expect("Obstacles is non-empty")
            })
            .max()
            .expect("Obstacles is non-empty");

        if floor {
            lowest_obstacle += 2;
        }

        Self {
            obstacles,
            start_point: Point::new(500, 0),
            sand: Default::default(),
            lowest_obstacle,
        }
    }

    fn is_blocked(&self, point: &Point) -> bool {
        self.sand.contains(point) || (self.obstacles.iter().any(|line| point.falls_on_line(line)))
    }

    fn is_blocked_by_floor(&self, point: &Point) -> bool {
        point.y + 1 == self.lowest_obstacle
    }

    fn drop_sand(&mut self) -> bool {
        if self.is_blocked(&self.start_point) {
            return false;
        }

        loop {
            if self.start_point.y > self.lowest_obstacle {
                return false;
            }

            if !self.is_blocked(&self.start_point.down()) {
                self.start_point = self.start_point.down();
            } else if !self.is_blocked(&self.start_point.down_left()) {
                self.start_point = self.start_point.down_left();
            } else if !self.is_blocked(&self.start_point.down_right()) {
                self.start_point = self.start_point.down_right();
            } else {
                let p = std::mem::replace(&mut self.start_point, Point::new(500, 0));
                self.sand.insert(p);
                return true;
            }
        }
    }

    fn drop_sand_with_floor(&mut self) -> bool {
        if self.is_blocked(&self.start_point) {
            return false;
        }

        loop {
            if self.is_blocked_by_floor(&self.start_point) {
                let p = std::mem::replace(&mut self.start_point, Point::new(500, 0));
                self.sand.insert(p);
                return true;
            }

            if !self.is_blocked(&self.start_point.down()) {
                self.start_point = self.start_point.down();
            } else if !self.is_blocked(&self.start_point.down_left()) {
                self.start_point = self.start_point.down_left();
            } else if !self.is_blocked(&self.start_point.down_right()) {
                self.start_point = self.start_point.down_right();
            } else {
                let p = std::mem::replace(&mut self.start_point, Point::new(500, 0));
                self.sand.insert(p);
                return true;
            }
        }
    }

    fn play(&mut self) {
        while self.drop_sand() {}
    }

    fn play_with_floor(&mut self) {
        while self.drop_sand_with_floor() {}
    }
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|point_slice| {
                    let (x, y) = point_slice
                        .split_once(',')
                        .expect("Point should have a comma");
                    Point::new(x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect()
}

pub mod p1 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let obstacles = parse(input);
        let mut game = Game::new(obstacles, false);
        game.play();
        game.sand.len()
    }
}

pub mod p2 {
    use super::*;
    pub fn solve(input: &str) -> usize {
        let obstacles = parse(input);
        let mut game = Game::new(obstacles, true);
        game.play_with_floor();
        game.sand.len()
    }
}

#[cfg(test)]
mod day14_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day14/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(p1::solve(SAMPLE), 24)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(p1::solve(input), 799)
    }

    #[test]
    fn p2_sample() {
        assert_eq!(p2::solve(SAMPLE), 93)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(p2::solve(input), 29076)
    }
}

#[cfg(test)]
mod day14_benchmarks {
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
