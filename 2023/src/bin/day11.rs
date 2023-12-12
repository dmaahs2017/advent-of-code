#![feature(test)]
extern crate test;
use aoc_2023::*;

use glam::IVec2;
use itertools::Itertools;
use std::collections::*;
const DAY: u8 = 11;

fn main() {
    let input = &read_input(DAY);
    println!(
        "Day {:0>2}: Part 1 answer = {}, Part 2 answer = {}",
        DAY,
        solve_p1(input),
        solve_p2(input)
    );
}

pub fn solve_p1(input: &str) -> usize {
    process(input, 2)

}

fn process(input: &str, scale: usize) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let expanded_rows: HashSet<i32> = lines
        .iter()
        .enumerate()
        .filter_map(|(y, line)| line.chars().all(|c| c == '.').then_some(y as i32))
        .collect();
    let expanded_cols: HashSet<i32> = (0..lines[0].len())
        .filter_map(|x| {
            (0..lines.len())
                .all(|y| lines[y].as_bytes()[x] == b'.')
                .then_some(x as i32)
        })
        .collect();

    let galaxy_positions = iter_absolute_galaxy_positions(&lines)
        .map(|pos| {
            let offset_y = (0..pos.y).filter(|y| expanded_rows.contains(y)).count() * ( scale -1 );
            let offset_x = (0..pos.x).filter(|x| expanded_cols.contains(x)).count() * ( scale -1 );
            let offset = IVec2::new(offset_x as i32, offset_y as i32);
            println!("pos {:?} + offset {:?} = {:?}", pos, offset, pos+offset);
            pos + offset
        })
        .collect_vec();

    (0..galaxy_positions.len() - 1).map(|i| {
        let a = galaxy_positions[i];
        (i+1..galaxy_positions.len()).map(|j| {
            let b = &galaxy_positions[j];
            let dist = (a.x - b.x).abs() + (a.y - b.y).abs();
            println!("{:?} - {:?} = {:?}", a, b, dist);
            dist as usize
        }).sum::<usize>()
    }).sum::<usize>() as usize
}

fn iter_absolute_galaxy_positions<'a>(lines: &'a Vec<&'a str>) -> impl Iterator<Item = IVec2> + 'a {
    lines.iter().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(x, c)| (c == '#').then_some(IVec2::new(x as i32, y as i32)))
    })
}

pub fn solve_p2(input: &str) -> usize {
    process(input, 1_000_000)
}

#[cfg(test)]
mod day11_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day11/sample.txt");

    #[test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 374)
    }

    #[test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 9418609)
    }

    #[test]
    fn p2_sample() {
        //assert_eq!(process(SAMPLE, 2), 1030);
        assert_eq!(process(SAMPLE, 10), 1030);
        assert_eq!(process(SAMPLE, 100), 8410);
    }

    #[test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 593821230983)
    }
}

#[cfg(test)]
mod day11_benchmarks {
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
