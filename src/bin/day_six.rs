use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file_path = "src/bin/inputs/day_six_input.txt";
    
    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("total yes count from groups: {}", part_one_result);
    
    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("total unanimous yes count from groups: {}", part_two_result);
}

fn execute_part_one(file_path: &str) -> usize {
    let groups = part_one_parse_input(file_path);
    let mut counter = 0;
    for group in groups {
        counter = counter + group.len();
    }

    return counter;
}

fn execute_part_two(file_path: &str) -> usize {
    let groups = part_two_parse_input(file_path);
    let mut counter = 0;
    for group in groups {
        counter = counter + count_unanimous_answers_for_group(group);
    }

    return counter;
}

fn part_one_parse_input(file_path:&str) -> Vec<Vec<String>> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    let mut group = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            result.push(group);
            group = vec![];
        }
        for character in line.chars() {
            if !group.contains(&character.to_string()) {
                group.push(character.to_string());
            }
        }
    }
    result.push(group);
    return result;
}

fn part_two_parse_input(file_path:&str) -> Vec<Vec<Vec<String>>> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    let mut group = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            result.push(group);
            group = vec![];
            continue;
        }
        let mut answers = vec![];
        for character in line.chars() {
            answers.push(character.to_string());
        }
        group.push(answers);
    }
    result.push(group);
    return result;
}

fn count_unanimous_answers_for_group(group: Vec<Vec<String>>) -> usize {
    let mut previous_answers = vec![];
    let mut counter = 0;
    for member in group {
        if counter == 0 {
            previous_answers = member.clone();
            counter = counter + 1;
            continue;
        }
        previous_answers = get_answer_intersection(previous_answers, member);
        counter = counter + 1;
    }

    return previous_answers.len();
}

fn get_answer_intersection(first_answers: Vec<String>, second_answers: Vec<String>) -> Vec<String> {
    let mut result = vec![];
    for answer in first_answers {
        if second_answers.contains(&answer) {
            result.push(answer);
        }
    }
    return result;
}

#[cfg(test)]
#[path = "tests/day_six_tests.rs"]
mod day_six_tests;