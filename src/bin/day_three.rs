use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let input_file_path = "src/bin/inputs/day_three_input.txt";

    // Part One
    let part_one_result = execute_part_one(input_file_path, vec![3,1]);
    println!("trees: {}", part_one_result);

    // Part Two
    let slopes = vec![
        vec![1,1],
        vec![3,1],
        vec![5,1],
        vec![7,1],
        vec![1,2]
    ];
    let part_two_result = execute_part_two(input_file_path, slopes);
    println!("trees: {}", part_two_result);
}

fn execute_part_two(file_path:&str, slopes:Vec<Vec<usize>>) -> usize {
    let mut results = vec![];
    for slope in slopes {
        results.push(execute_part_one(file_path, slope));
    }

    let mut total_trees = 1;
    for trees in results {
        total_trees = total_trees * trees;
    }

    return total_trees;
}

fn execute_part_one(file_path:&str, slope:Vec<usize>) -> usize {
    let grid = parse_input(file_path);
    let horizontal_width = grid[0].len();
    let horizontal_step = slope[0];
    let vertical_step = slope[1];
    let mut horizontal_position = 0;
    let mut vertical_position = 0;
    
    let done = false;
    let mut counter = 0;
    while !done {
        let row_number = increment_vertical(vertical_position, vertical_step);
        let column_number = increment_horizontal(horizontal_position, horizontal_step, horizontal_width);
        if row_number >= grid.len() {
            break;
        }
        let row = &grid[row_number];
        let square = &row[column_number];
        if square == "#" {
            counter = counter + 1;
        }

        horizontal_position = column_number;
        vertical_position = row_number;
    }

    return counter;
}

fn increment_horizontal(position:usize, step:usize, width:usize) -> usize {
    if position + step >= width {
        return position + step - width;
    }
    return position + step;
}

fn increment_vertical(position:usize, step:usize) -> usize {
    return position + step;
}

fn parse_input(file_path:&str) -> Vec<Vec<String>> {
    let reader = BufReader::new(File::open(file_path).expect("Cannot open file"));
    let mut result = vec![];
    for line in reader.lines() {
        let mut line_vector = vec![];
        for part in line.unwrap().chars() {
            line_vector.push(String::from(part));
        }
        result.push(line_vector);
    }

    return result;
}

#[cfg(test)]
#[path = "tests/day_three_tests.rs"]
mod day_three_tests;