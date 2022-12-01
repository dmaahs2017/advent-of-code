fn main() {
    let input = include_str!("../../inputs/day1/input.txt");
    println!("The elf with the most calories carries {} calories", solve(input))
}

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn solve(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .expect("input was empty")
}

#[test]
fn it_works() {
    let input = include_str!("../../inputs/day1/sample.txt");
    assert_eq!(solve(input), 24000)
}
