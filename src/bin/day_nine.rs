use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let input_file_path = "src/bin/inputs/day_nine_input.txt";

    let mut now = Instant::now();
    // Part One
    let part_one_result = execute_part_one(input_file_path, 25, 25);
    println!("({} ms) first non-sum entry: {}", now.elapsed().as_millis(), part_one_result);

    now = Instant::now();
    // Part Two
    let part_two_result = execute_part_two(input_file_path, part_one_result);
    println!("({} ms) encryption weakness: {}", now.elapsed().as_millis(), part_two_result);
}

fn execute_part_one(file_path:&str, look_back_length: usize, preamble_length: usize) -> usize {
    let lines = parse_input(file_path);
    let mut sequence = vec![];
    for line in lines {
        sequence.push(line.parse::<usize>().unwrap());
    }

    return get_first_non_sum_entry(sequence, look_back_length, preamble_length)
}

fn execute_part_two(file_path:&str, target_value: usize) -> usize {
    let lines = parse_input(file_path);
    let mut sequence = vec![];
    for line in lines {
        sequence.push(line.parse::<usize>().unwrap());
    }

    let summation_series = get_series_summation_equaling_target(sequence, target_value);
    return summation_series.iter().min().unwrap() + summation_series.iter().max().unwrap();
}

fn get_first_non_sum_entry(sequence: Vec<usize>, look_back_length: usize, preamble_length: usize) -> usize {
    let result = 0;
    for index in 0..sequence.len() {
        if index < preamble_length {
            continue;
        }
        let sliced_sequence = &sequence[(index - look_back_length)..index];
        let sum_values = get_summation_values(sliced_sequence.to_vec(), sequence.clone()[index]);

        if sum_values.len() <= 0 {
            return sequence.clone()[index];
        }
    }

    return result;
}

fn get_summation_values(sequence: Vec<usize>, next_value: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for number_one in sequence.clone() {
        for number_two in sequence.clone() {
            if number_one + number_two == next_value && number_one != number_two {
                let pair =  if number_one > number_two {(number_one, number_two)} else {(number_two, number_one)};
                if !result.contains(&pair) {
                    result.push(pair);
                }
            }
        }
    }

    return result;
}

fn get_series_summation_equaling_target(input_sequence: Vec<usize>, target_value: usize) -> Vec<usize> {
    for start in 0..input_sequence.len() {
        for end in 0..input_sequence.len() {
            if start >= end {
                continue;
            }

            let test_sequence = &input_sequence.clone()[start..=end];
            if test_sequence.iter().sum::<usize>() == target_value {
                return test_sequence.to_vec();
            }
        }
    }

    return vec![];
}

fn parse_input(file_path:&str) -> Vec<String> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        result.push(line);
    }

    return result;
}

#[cfg(test)]
#[path = "tests/day_nine_tests.rs"]
mod day_nine_tests;