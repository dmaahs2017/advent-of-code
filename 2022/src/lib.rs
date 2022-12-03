use std::fs;
use std::io::read_to_string;

pub fn read_input(day: u8) -> String {
    let f =
        fs::File::open(format!("inputs/day{:0>2}/input.txt", day)).expect("input.txt not found");
    read_to_string(f).expect("Failed to read input file to string")
}

pub fn read_sample(day: u8) -> String {
    let f =
        fs::File::open(format!("inputs/day{:0>2}/sample.txt", day)).expect("input.txt not found");
    read_to_string(f).expect("Failed to read input file to string")
}
