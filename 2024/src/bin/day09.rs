#![feature(test)]
extern crate test;
use aoc_2024::*;

const DAY: u8 = 9;

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

#[derive(Debug, Copy, Clone)]
struct File {
    start: usize,
    len: usize,
    id: usize,
}

#[derive(Debug, Copy, Clone)]
struct Free {
    len: usize,
    start: usize,
}

#[derive(Debug, Copy, Clone)]
enum Segment {
    File(File),
    Free(Free),
}

fn parse(s: &str) -> Vec<Segment> {
    let mut index = 0;
    s.chars()
        .enumerate()
        .filter_map(|(i, c)| {
            if c == '\n' {
                return None;
            }
            let len = c.to_digit(10).expect("Valid digit") as usize;
            let bt = if i % 2 == 0 {
                Segment::File(File {
                    len,
                    id: i / 2,
                    start: index,
                })
            } else {
                Segment::Free(Free { len, start: index })
            };
            index += len;
            Some(bt)
        })
        .collect()
}

#[tracing::instrument(skip(input))]
pub fn solve_p1(input: &str) -> usize {
    let summary = parse(input);

    let mut memory = summary
        .iter()
        .flat_map(|b| match *b {
            Segment::File(fs) => (0..fs.len).map(|_| Some(fs.id)).collect::<Vec<_>>(),
            Segment::Free(free) => (0..free.len).map(|_| None).collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    let mut lptr = 0;
    let mut rptr = memory.len() - 1;

    while lptr < rptr {
        if memory[lptr].is_some() {
            lptr += 1;
            continue;
        }
        if memory[rptr].is_none() {
            rptr -= 1;
            continue;
        }

        memory.swap(lptr, rptr);
    }

    checksum(&memory)
}

#[tracing::instrument(skip(input))]
pub fn solve_p2(input: &str) -> usize {
    let summary = parse(input);

    let mut free_segments = vec![];
    let mut file_segments = vec![];
    for s in summary.iter() {
        match *s {
            Segment::File(f) => file_segments.push(f),
            Segment::Free(f) => free_segments.push(f),
        }
    }

    let mut memory = summary
        .iter()
        .flat_map(|b| match *b {
            Segment::File(fs) => (0..fs.len).map(|_| Some(fs.id)).collect::<Vec<_>>(),
            Segment::Free(free) => (0..free.len).map(|_| None).collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    for file in file_segments.iter().rev() {
        let Some(free) = free_segments
            .iter_mut()
            .find(|free| free.start < file.start && file.len <= free.len)
        else {
            continue;
        };

        for block in memory.iter_mut().skip(free.start).take(file.len) {
            *block = Some(file.id);
        }
        for block in memory.iter_mut().skip(file.start).take(file.len) {
            *block = None;
        }
        free.start += file.len;
        free.len -= file.len;
    }

    checksum(&memory)
}

fn checksum(mem: &[Option<usize>]) -> usize {
    mem.iter()
        .enumerate()
        .filter_map(|(i, opt)| opt.as_ref().map(|id| i * id))
        .sum()
}

#[cfg(test)]
mod day09_tests {
    use super::*;

    const SAMPLE: &str = include_str!("../../inputs/day09/sample.txt");

    #[test_log::test]
    fn p1_sample() {
        assert_eq!(solve_p1(SAMPLE), 1928)
    }

    #[test_log::test]
    fn p1_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p1(input), 6279058075753)
    }

    #[test_log::test]
    fn p2_sample() {
        assert_eq!(solve_p2(SAMPLE), 2858)
    }

    #[test_log::test]
    fn p2_input() {
        let input = &read_input(DAY);
        assert_eq!(solve_p2(input), 6301361958738)
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
        b.iter(|| solve_p1(input))
    }

    #[bench]
    #[ignore]
    fn bench_p2(b: &mut Bencher) {
        let input = &read_input(DAY);
        b.iter(|| solve_p2(input))
    }
}
