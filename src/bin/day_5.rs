use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file_path = "src/bin/inputs/day_five_input.txt";
    
    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("highest seat id: {}", part_one_result);
    
    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("my seat id: {}", part_two_result);
}

fn execute_part_two(file_path: &str) -> usize {
    let lines = parse_input(file_path);
    let mut seats = vec![vec![]; 127];
    for line in lines {
        let row = find_row(&line[..7]);
        let column = find_column(&line[7..]);
        seats[row].push(column);
    }
    let mut missing_row = 0;
    let mut missing_column = 0;
    let mut counter = 0;
    for mut seat in seats {
        if seat.len() <= 6 {
            counter = counter + 1;
            continue;
        }
        if seat.len() < 8 {
            seat.sort();
            let mut previous = 0;
            for column in seat {
                if previous == 0 {
                    previous = column;
                    continue;
                }
                if column != previous + 1 {
                    missing_row = counter;
                    missing_column = column - 1;
                    return get_seat_id(missing_row, missing_column);
                }

            }
        }

        counter = counter + 1;
    }

    return get_seat_id(missing_row, missing_column);
}

fn execute_part_one(file_path: &str) -> usize {
    let lines = parse_input(file_path);
    let mut highest = 0;
    for line in lines {
        let row = find_row(&line[..7]);
        let column = find_column(&line[7..]);
        let seat_id = get_seat_id(row, column);
        if seat_id > highest {
            highest = seat_id;
        }
    }

    return highest;
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

fn find_row(directions: &str) -> usize {
    let mut vector : Vec<usize> = (0..128).map(|x| x as usize).collect();
    for direction in directions.chars() {
        vector = take_half(direction, vector);
    }

    return vector[0];
}

fn find_column(directions: &str) -> usize {
    let mut vector : Vec<usize> = (0..8).map(|x| x as usize).collect();
    for direction in directions.chars() {
        vector = take_half(direction, vector);
    }

    return vector[0];
}

fn get_seat_id(row: usize, column: usize) -> usize {
    return (row * 8) + column;
}

fn take_half(direction: char, mut vector: Vec<usize>) -> Vec<usize> {
    let back = vector.split_off(vector.len()/2);
    if direction == 'F' || direction == 'L' {
        return vector;
    }
    if direction == 'B' || direction == 'R' {
        return back;
    }
    return vec![];
}

#[cfg(test)]
#[path = "tests/day_5_tests.rs"]
mod day_5_tests;