#![feature(test)]
extern crate test;

fn main() {
    let input = include_str!("../../inputs/day1/input.txt");
    println!(
        "The elf with the most calories carries {} calories",
        solve_puzzle_1(input)
    );
    println!(
        "The sum of 3 elves with the most calories is {}",
        solve_puzzle_2(input)
    );
}

/// Convert input &str into an iterator over each elf's total calories
fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.split("\n\n").map(|elf| {
        elf.split_whitespace()
            .map(|c| c.parse::<usize>().unwrap())
            .sum()
    })
}

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn solve_puzzle_1(input: &str) -> usize {
    parse_input(input).max().expect("input was empty")
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn solve_puzzle_2(input: &str) -> usize {
    parse_input(input)
        .fold([0, 0, 0], |mut acc, elf_bag: usize| {
            let min = acc.iter_mut().min().unwrap();
            *min = elf_bag.max(*min);
            acc
        })
        .iter()
        .sum()
}

#[cfg(test)]
fn solve_puzzle_2_with_sorting(input: &str) -> usize {
    let mut v = parse_input(input).collect::<Vec<_>>();
    v.sort();
    v.iter().rev().take(3).sum()
}

#[cfg(test)]
mod day1_tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_puzzle_2(b: &mut Bencher) {
        let input = include_str!("../../inputs/day1/sample.txt");
        b.iter(|| solve_puzzle_2(input))
    }

    #[bench]
    fn bench_puzzle_2_with_sorting(b: &mut Bencher) {
        let input = include_str!("../../inputs/day1/sample.txt");
        b.iter(|| solve_puzzle_2_with_sorting(input))
    }

    #[test]
    fn solve_puzzle_2_works() {
        let input = include_str!("../../inputs/day1/sample.txt");
        assert_eq!(solve_puzzle_2(input), 45000)
    }

    #[test]
    fn solve_puzzle_1_works() {
        let input = include_str!("../../inputs/day1/sample.txt");
        assert_eq!(solve_puzzle_1(input), 24000)
    }
}

