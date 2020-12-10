use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let input_file_path = "src/bin/inputs/day_four_input.txt";
    
    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("passports with required fields present: {}", part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("passports with required fields present and valid: {}", part_two_result);
}

fn execute_part_one(file_path:&str) -> usize {
    let passports = parse_input(file_path);
    let mut counter = 0;
    for passport in passports {
        if is_passport_fields_present(&passport) {
            counter = counter + 1;
        }
    }

    return counter;
}

fn execute_part_two(file_path:&str) -> usize {
    let passports = parse_input(file_path);
    let mut counter = 0;
    for passport in passports {
        if is_passport_fields_present(&passport) && is_passport_fields_valid(&passport) {
            counter = counter + 1;
        }
    }

    return counter;
}

fn is_passport_fields_valid(passport:&Vec<String>) -> bool {
    for field in passport {
        if field.starts_with("byr") && !is_year_valid(field, 1920, 2002) {
            return false;
        }
        if field.starts_with("iyr") && !is_year_valid(field, 2010, 2020) {
            return false;
        }
        if field.starts_with("eyr") && !is_year_valid(field, 2020, 2030) {
            return false;
        }
        if field.starts_with("hgt") && !is_height_valid(field) {
            return false;
        }
        if field.starts_with("hcl") && !is_hair_color_valid(field) {
            return false;
        }
        if field.starts_with("ecl") && !is_eye_color_valid(field) {
            return false;
        }
        if field.starts_with("pid") && !is_passport_id_valid(field) {
            return false;
        }
    }
    return true;
}

fn parse_input(file_path:&str) -> Vec<Vec<String>> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut lines = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = vec![];
        for part in split_line(" ", &line) {
            parts.push(String::from(part));
        }
        lines.push(parts);
    }

    let mut result = vec![];
    let mut parts = vec![];
    for line in lines {
        if line == vec![""] {
            result.push(parts);
            parts = vec![];
            continue;
        }
        for part in line {
            parts.push(String::from(part));
        }
    }
    result.push(parts);
    return result;
}

fn is_passport_fields_present(passport:&Vec<String>) -> bool {
    if passport.len() < 7 {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("byr")) {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("iyr")) {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("eyr")) {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("hgt")) {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("hcl")) {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("ecl")) {
        return false;
    }
    if !passport.iter().any(|i| i.starts_with("pid")) {
        return false;
    }
    return true;
}

fn is_year_valid(year: &String, min: usize, max: usize) -> bool {
    let parts = split_line(":", year);
    let value = parts[1].parse::<usize>().unwrap();
    if value < min || value > max {
        return false;
    }
    return true;
}

fn is_height_valid(hgt: &String) -> bool {
    let parts = split_line(":", hgt);
    let value = parts[1].parse::<String>().unwrap();
    let rgx = Regex::new("(^(1[5-8][0-9]|19[0-3])cm\\b|\\b(59|6[0-9]|7[0-6])in$)").unwrap();
    return rgx.is_match(&value);
}

fn is_hair_color_valid(hcl: &String) -> bool {
    let parts = split_line(":", hcl);
    let value = parts[1].parse::<String>().unwrap();
    let rgx = Regex::new("(^#([a-f0-9]){6}$)").unwrap();
    return rgx.is_match(&value);
}

fn is_eye_color_valid(ecl: &String) -> bool {
    let parts = split_line(":", ecl);
    let value = parts[1].parse::<String>().unwrap();
    let valid = vec![
        "amb".to_string(), 
        "blu".to_string(), 
        "brn".to_string(), 
        "gry".to_string(), 
        "grn".to_string(), 
        "hzl".to_string(), 
        "oth".to_string()
    ];
    return valid.contains(&value);
}

fn is_passport_id_valid(pid: &String) -> bool {
    let parts = split_line(":", pid);
    let value = parts[1].parse::<String>().unwrap();
    let rgx = Regex::new("(^([0-9]){9}$)").unwrap();
    return rgx.is_match(&value);
}

fn split_line<'a>(separator: &'a str, line: &'a str) -> Vec<&'a str> {
    return line.split(separator).collect::<Vec<&str>>();
}

#[cfg(test)]
#[path = "tests/day_4_tests.rs"]
mod day_4_tests;