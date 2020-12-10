use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn main() {}

#[allow(dead_code)]
pub fn parse_strings(file_path: &str) -> Vec<String> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line);
    }

    return result;
}

#[allow(dead_code)]
pub fn parse_ints(file_path: &str) -> Vec<usize> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line.parse::<usize>().unwrap());
    }

    return result;
}