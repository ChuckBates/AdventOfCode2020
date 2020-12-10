use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::str;

fn main() {
    let input_file_path = "src/bin/inputs/day_eight_input.txt";

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("accumulator before repeat: {}", part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("accumulator without corruption: {}", part_two_result);
}

fn execute_part_one(file_path:&str) -> isize {
    let lines = parse_input(file_path);
    let mut accumulator = 0;
    let mut position: isize = 0;
    let mut previous_positions = HashSet::new();
    previous_positions.insert(position);
    loop {
        if previous_positions.len() >= lines.len() {
            break;
        }
        let step = &lines[position as usize];
        let result = execute_step(step.clone(), accumulator, position);
        let new_position = result.1;
        if previous_positions.contains(&new_position) && !step.starts_with("nop") {
            break;
        }
        previous_positions.insert(result.1);
        accumulator = result.0;
        position = result.1;
    }

    return accumulator;
}

fn get_new_accumulator(lines: Vec<String>) -> isize {
    let lines = lines.clone();
    let mut accumulator = 0;
    let mut position: isize = 0;
    let mut previous_positions = HashSet::new();
    previous_positions.insert(position);
    loop {
        if position >= lines.len() as isize {
            break;
        }
        let step = &lines[position as usize];
        let result = execute_step(step.clone(), accumulator, position);
        let new_position = result.1;
        if previous_positions.contains(&new_position) && !step.starts_with("nop") {
            return -999;
        }
        previous_positions.insert(result.1);
        accumulator = result.0;
        position = result.1;
    }

    return accumulator;
}

fn execute_part_two(file_path:&str) -> isize {
    let lines = parse_input(file_path);
    let accumulator = 0;

    for line in lines.clone() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let index = lines.clone().iter().position(|r| r == &line).unwrap();
        let op = parts[0].parse::<String>().unwrap();
        let increment = parts[1].parse::<isize>().unwrap();
        let increment_str = parts[1];
        if op == "nop" && increment != 0 {
            let mut new_lines = lines.clone();
            new_lines[index] = "jmp ".to_owned() + increment_str;
            let new_accumulator = get_new_accumulator(new_lines.clone());
            if new_accumulator != -999 {
                return new_accumulator;
            }
        }

        if op == "jmp" {
            let mut new_lines = lines.clone();
            new_lines[index] = "nop +0".to_string();
            let new_accumulator = get_new_accumulator(new_lines.clone());
            if new_accumulator != -999 {
                return new_accumulator;
            }
        }
    }

    return accumulator;
}

fn execute_step(step: String, accumulator: isize, position: isize) -> (isize, isize) {
    let mut accumulator_value = accumulator;
    let mut position_value = position;
    let parts = step.split(" ").collect::<Vec<&str>>();
    let op = parts[0].parse::<String>().unwrap();
    println!("increment: {:?}", parts);
    let increment = parts[1].parse::<isize>().unwrap();
    if op == "acc" {
        accumulator_value = accumulator_value + increment;
        position_value = position_value + 1;
    } else if op == "jmp" {
        position_value = position_value + increment;
    } else if op == "nop" {
        position_value = position_value + 1;
    }

    return (accumulator_value, position_value);
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
#[path = "tests/day_eight_tests.rs"]
mod day_eight_tests;