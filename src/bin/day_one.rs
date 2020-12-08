use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file_path = "src/bin/inputs/day_one_input.txt";

    //Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("product of two entries: {}", part_one_result);

    //Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("product of three entries: {}", part_two_result);
}

fn execute_part_one(file_path:&str) -> usize {
    let lines = parse_input(file_path);
    for line_x in lines.clone() {
        for line_y in lines.clone() {
            let x = line_x.parse::<usize>().unwrap();
            let y = line_y.parse::<usize>().unwrap();
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    return 0;
}

fn execute_part_two(file_path:&str) -> usize {
    let lines = parse_input(file_path);
    for line_x in lines.clone() {
        for line_y in lines.clone() {
            for line_z in lines.clone() {
                let x = line_x.parse::<usize>().unwrap();
                let y = line_y.parse::<usize>().unwrap();
                let z = line_z.parse::<usize>().unwrap();
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    return 0;
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
#[path = "tests/day_one_tests.rs"]
mod day_one_tests;