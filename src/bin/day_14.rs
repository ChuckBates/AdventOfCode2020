use std::time::Instant;
use std::collections::HashMap;
extern crate itertools;
use itertools::Itertools;
mod input;

fn main() {
    let input_file_path = "src/bin/inputs/day_14_input.txt";
    let now = Instant::now();

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("({} seconds) sum of non-zero values: {}", now.elapsed().as_secs_f32(), part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("({} seconds) sum of non-zero values: {}", now.elapsed().as_secs_f32(), part_two_result);
}

fn execute_part_one(file_path: &str) -> usize {
    let lines = input::parse_strings(file_path);
    let result = execute_blocks_part_one(lines);

    let non_zero_values = result.values().filter(|&m| m > &0);
    return non_zero_values.sum();
}

fn execute_part_two(file_path: &str) -> usize {
    let lines = input::parse_strings(file_path);
    let result = execute_blocks_part_two(lines);

    let non_zero_values = result.values().filter(|&m| m > &0);
    return non_zero_values.sum();
}

fn execute_blocks_part_one(lines: Vec<String>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize,usize> = HashMap::new();
    let mut mask = "".to_string();
    for line in lines {
        if line.starts_with("mask") {
            mask = line[7..].to_string();
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        let value = parts[2].parse::<usize>().unwrap();
        let address = parts[0][4..parts[0].len()-1].parse::<usize>().unwrap();
        result.insert(address, mutate_value_with_mask(value, mask.clone()));
    }

    return result;
}

fn execute_blocks_part_two(lines: Vec<String>) -> HashMap<usize, usize> {
    let mut result: HashMap<usize,usize> = HashMap::new();
    let mut mask = "".to_string();
    for line in lines {
        if line.starts_with("mask") {
            mask = line[7..].to_string();
            continue;
        }
        let parts = line.split(" ").collect::<Vec<&str>>();
        let value = parts[2].parse::<usize>().unwrap();
        let address = parts[0][4..parts[0].len()-1].parse::<usize>().unwrap();
        let permutations = mutate_address_with_mask(address, mask.clone());
        for permutation in permutations {
            result.insert(permutation, value);
        }
    }

    return result;
}

fn mutate_address_with_mask(address: usize, mask: String) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    let address_as_binary = format!("{:b}", address);
    let difference = mask.len() - address_as_binary.len();
    let mut rev_result = "".to_string();
    let mut x_count = 0;
    for index in (0..36).rev() {
        let bit_mask = mask.chars().nth(index).unwrap();
        let mut new_value = "0".to_string();

        if index >= difference {
            if bit_mask.to_string() == "1" {
                new_value = "1".to_string();
            } else if bit_mask.to_string() == "X" { 
                new_value = "X".to_string();
                x_count += 1;
            } else {
                new_value = address_as_binary.chars().nth(index - difference).unwrap().to_string();
            }
        } else {
            if bit_mask.to_string() == "1" {
                new_value = "1".to_string();
            } else if bit_mask.to_string() == "X" {
                new_value = "X".to_string();
                x_count += 1;
            }
        }

        rev_result = format!("{}{}", rev_result, new_value);
    }

    let address_as_string = rev_result.chars().rev().collect::<String>();
    if x_count > 0 {
        let address_permutations = fill_binary_combinations_into_result(get_all_binary_combinations(x_count), address_as_string);
        for permutation in address_permutations {
            result.push(usize::from_str_radix(&permutation, 2).unwrap());
        }
    } else {
        let address_as_usize = usize::from_str_radix(&address_as_string, 2).unwrap();
        result.push(address_as_usize);
    }
    return result;
}

fn get_all_binary_combinations(count: usize) -> Vec<Vec<usize>> {
    let mut result = vec![];
    for _ in 0..count {
        result.push(0);
        result.push(1);
    }
    let mut values = vec![];
    for value in result.into_iter().combinations(count).unique() {
        values.push(value);
    }
    return values;
}

fn fill_binary_combinations_into_result(combos: Vec<Vec<usize>>, result: String) -> Vec<String> {
    let mut new_results = vec![];
    for combo in combos {
        let mut mutable_result = result.clone();
        for value in combo {
            mutable_result = mutable_result.clone().replacen("X", &value.to_string(), 1);
        }
        new_results.push(mutable_result.clone());
    }
    return new_results;
}

fn mutate_value_with_mask(value: usize, mask: String) -> usize {
    let value_as_binary = format!("{:b}", value);
    let difference = mask.len() - value_as_binary.len();
    let mut rev_result = "".to_string();
    for index in (0..36).rev() {
        let bit_mask = mask.chars().nth(index).unwrap();
        let mut new_value = "0".to_string();

        if index >= difference {
            if bit_mask.to_string() != "X" {
                new_value = bit_mask.to_string();
            } else {
                new_value = value_as_binary.chars().nth(index - difference).unwrap().to_string();
            }
        } else {
            if bit_mask.to_string() != "X" {
                new_value = bit_mask.to_string();
            }
        }

        rev_result = format!("{}{}", rev_result, new_value);
    }

    let result = rev_result.chars().rev().collect::<String>();

    return usize::from_str_radix(&result, 2).unwrap();
}

#[cfg(test)]
#[path = "tests/day_14_tests.rs"]
mod day_14_tests;