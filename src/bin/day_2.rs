use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file_path = "src/bin/inputs/day_two_input.txt";

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("valid passwords: {}", part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("valid passwords: {}", part_two_result);
}

fn execute_part_one(file_path:&str) -> usize {
    let lines = parse_input(file_path);
    let mut counter = 0;
    for line in lines {
        let parts = split_line(":", &line);
        let password = parts[1];
        let range = split_line(" ", parts[0]);
        let required_character = range[1];
        let limits = split_line("-", range[0]);
        let lower_limit = limits[0].parse::<usize>().unwrap();
        let upper_limit = limits[1].parse::<usize>().unwrap();

        if password.contains(required_character) {
            let number_of_occurances = password.matches(required_character).count();
            if number_of_occurances >= lower_limit && number_of_occurances <= upper_limit {
                counter = counter + 1;
            }
        }
    }

    return counter;
}

fn execute_part_two(file_path:&str) -> usize {
    let lines = parse_input(file_path);
    let mut counter = 0;
    for line in lines {
        let parts = split_line(":", &line);
        let password = parts[1];
        let range = split_line(" ", parts[0]);
        let required_character = range[1];
        let indcies = split_line("-", range[0]);
        let first_index = indcies[0].parse::<usize>().unwrap();
        let second_index = indcies[1].parse::<usize>().unwrap();

        if password.contains(required_character) && first_index > 0 {
            let first_index_char = password.chars().nth(first_index).unwrap();
            let second_index_char = password.chars().nth(second_index).unwrap();
            let required_character = required_character.chars().nth(0).unwrap();
            if (first_index_char == required_character) ^ (second_index_char == required_character) {
                counter = counter + 1;
            }
        }
    }

    return counter;
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

fn split_line<'a>(separator: &'a str, line: &'a str) -> Vec<&'a str> {
    return line.split(separator).collect::<Vec<&str>>();
}

#[cfg(test)]
#[path = "tests/day_2_tests.rs"]
mod day_2_tests;