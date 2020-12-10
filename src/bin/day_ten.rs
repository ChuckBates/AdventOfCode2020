use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let input_file_path = "src/bin/inputs/day_ten_input.txt";
    let now = Instant::now();

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("({} ms) product of jolt differences: {}", now.elapsed().as_millis(), part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("({} ms) possible adapter combinations: {}", now.elapsed().as_micros(), part_two_result);
    
}

fn execute_part_two(file_path: &str) -> usize {
    let mut adapters = parse_input(file_path);
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();
    
    let mut look_backs = vec![1];
    for index_outer in 1..adapters.len() {
        let mut number_of_valid_combinations = 0;
        for index_inner in 0..index_outer {
            if adapters[index_inner] + 3 >= adapters[index_outer] {
                number_of_valid_combinations += look_backs[index_inner];
            }
        }
        look_backs.push(number_of_valid_combinations);
    }
    
    return *look_backs.last().unwrap();
}

fn execute_part_one(file_path: &str) -> usize {
    let mut adapters = parse_input(file_path);
    adapters.sort();
    let mut three_jolt_jumps = 1;
    let mut one_jolt_jumps = 0;
    if adapters[0] == 3 { three_jolt_jumps += 1;}
    if adapters[0] == 1 { one_jolt_jumps += 1;}
    for index in 0..adapters.len()-1 {
        let difference = adapters[index + 1] - adapters[index];
        if difference == 3 {
            three_jolt_jumps += 1;
        }
        if difference == 1 {
            one_jolt_jumps += 1;
        }
        if difference > 3 || difference < 1 {
            return 0;
        }
    }

    return three_jolt_jumps * one_jolt_jumps;
}

fn parse_input(file_path:&str) -> Vec<usize> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line.parse::<usize>().unwrap());
    }

    return result;
}

#[cfg(test)]
#[path = "tests/day_ten_tests.rs"]
mod day_ten_tests;