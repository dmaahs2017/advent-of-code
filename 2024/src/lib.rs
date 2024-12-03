use std::collections::HashSet;
use std::fs;
use std::io::read_to_string;
use std::io::Write;

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

pub fn write_file(s: &str, file_name: &str) {
    let mut f = fs::File::create(file_name).expect("Faile&d to create file");
    write!(&mut f, "{}", s).expect("Failed to write file")
}

/// In place intersection of two hash sets
pub fn intersect<T: Eq + std::hash::Hash>(s1: &mut HashSet<T>, s2: &HashSet<T>) {
    s1.retain(|s| s2.contains(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn intersect_works() {
        let mut s1 = HashSet::from(['a', 'b', 'c']);
        let s2 = HashSet::from(['c', 'd', 'e']);
        intersect(&mut s1, &s2);
        assert_eq!(s1.into_iter().collect::<Vec<_>>(), vec!['c']);
    }
}
