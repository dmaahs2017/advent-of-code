#![feature(test)]
extern crate test;
use aoc_2023::*;

const DAY: u8 = 10;

use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    use Tile::*;
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '|' => NorthSouth,
                    '-' => EastWest,
                    'L' => NorthEast,
                    'J' => NorthWest,
                    '7' => SouthWest,
                    'F' => SouthEast,
                    '.' => Ground,
                    'S' => Start,
                    x => unreachable!("No other type of tyle should exist: {}", x),
                })
                .collect()
        })
        .collect()
}

pub fn solve_p1(input: &str) -> usize {
    let map = parse(input);
    traverse(&map).len() / 2
}

fn get_tile(map: &[Vec<Tile>], pos: IVec2) -> Option<Tile> {
    map.get(pos.y as usize)
        .and_then(|row| row.get(pos.x as usize))
        .copied()
}

fn is_connected(map: &[Vec<Tile>], pos: IVec2, adj_pos: IVec2) -> bool {
    use Tile::*;
    let offset = adj_pos - pos;
    let cur_tile = get_tile(map, pos);
    let adj_tile = get_tile(map, adj_pos);

    let west_pipes = [NorthWest, EastWest, SouthWest, Start].map(Some);
    let east_pipes = [EastWest, NorthEast, SouthEast, Start].map(Some);
    let north_pipes = [NorthEast, NorthWest, NorthSouth, Start].map(Some);
    let south_pipes = [SouthEast, SouthWest, NorthSouth, Start].map(Some);

    match offset {
        IVec2 { x: -1, y: 0 } => west_pipes.contains(&cur_tile) && east_pipes.contains(&adj_tile),
        IVec2 { x: 1, y: 0 } => east_pipes.contains(&cur_tile) && west_pipes.contains(&adj_tile),
        IVec2 { x: 0, y: 1 } => south_pipes.contains(&cur_tile) && north_pipes.contains(&adj_tile),
        IVec2 { x: 0, y: -1 } => north_pipes.contains(&cur_tile) && south_pipes.contains(&adj_tile),
        _ => unreachable!("Should only be looking at adj pipes"),
    }
}

fn traverse(map: &[Vec<Tile>]) -> Vec<IVec2> {
    let mut position = map
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, tile)| (*tile == Tile::Start).then_some(x))
                .map(|x| IVec2::new(x as i32, y as i32))
        })
        .unwrap();

    let mut visited: Vec<IVec2> = vec![];
    loop {
        let connections = [
            IVec2::new(-1, 0),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(0, 1),
        ]
        .into_iter()
        .map(|offset| position + offset)
        .filter(|p| is_connected(map, position, *p) && !visited.contains(p))
        .collect_vec();

        visited.push(position);

        if connections.is_empty() {
            return visited;
        }

        position = connections[0];
    }
}

pub fn solve_p2(input: &str) -> usize {
    use Tile::*;
    let map = parse(input);
    let pipe_loop = traverse(&map);

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|&(x, _)| {
                    let cur_pos = IVec2::new(x as i32, y as i32);
                    if pipe_loop.contains(&cur_pos) {
                        return false;
                    }

                    // if you cast a ray, that is non colinear to any of the shapes borders, from any point. That point is within the
                    // enclosed shape if the number of intersections is odd
                    let ray_cast_intersection_count = (1..)
                        .zip(1..)
                        .map(|(x, y)| cur_pos + IVec2::new(x, y))
                        .take_while(|p| p.y < map.len() as i32 && p.x < map[0].len() as i32)
                        .filter_map(|pos| Some((pos, get_tile(&map, pos)?)))
                        .filter(|(pos, tile)| {
                            pipe_loop.contains(pos) && *tile != NorthEast && *tile != SouthWest
                        })
                        .count();

                    ray_cast_intersection_count % 2 == 1
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod day10_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day10/sample.txt");
    const SAMPLE_2: &str = include_str!("../../inputs/day10/sample_2.txt");
    const SAMPLE_3: &str = include_str!("../../inputs/day10/sample_3.txt");
    const SAMPLE_4: &str = include_str!("../../inputs/day10/sample_4.txt");
    const SAMPLE_5: &str = include_str!("../../inputs/day10/sample_5.txt");
    const SAMPLE_6: &str = include_str!("../../inputs/day10/sample_6.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 4)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 6733)
    }

    #[test]
    fn p2_sample_2() {
        assert_eq!(solve_p2(SAMPLE_2), 4)
    }

    #[test]
    fn p2_sample_3() {
        assert_eq!(solve_p2(SAMPLE_3), 4)
    }

    #[test]
    fn p2_sample_4() {
        assert_eq!(solve_p2(SAMPLE_4), 8)
    }

    #[test]
    fn p2_sample_5() {
        assert_eq!(solve_p2(SAMPLE_5), 10)
    }

    #[test]
    fn p2_sample_6() {
        assert_eq!(solve_p2(SAMPLE_6), 0)
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 435)
    }
}

#[cfg(test)]
mod day10_benchmarks {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p1(input))
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
