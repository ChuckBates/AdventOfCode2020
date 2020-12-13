use std::time::Instant;
use std::convert::TryInto;
mod input;

fn main() {
    let input_file_path = "src/bin/inputs/day_12_input.txt";
    let now = Instant::now();

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("({} seconds) distance: {}", now.elapsed().as_secs_f32(), part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("({} seconds) distance: {}", now.elapsed().as_secs_f32(), part_two_result);
}

fn execute_part_one(file_path: &str) -> isize {
    let lines = input::parse_strings(file_path);
    let mut position = (0,0);
    let mut direction = "E".to_string();
    for line in lines {
        let result = handle_instruction(line, direction.to_string(), position.clone());
        position = (result.clone().0, result.clone().1);
        direction = result.clone().2.to_string();
    }

    return position.0.abs() + position.1.abs();
}

fn execute_part_two(file_path: &str) -> isize {
    let lines = input::parse_strings(file_path);
    let mut ship_position = (0,0);
    let mut waypoint_position = (10,1);
    for line in lines {
        let result = handle_instruction_part_two(line, ship_position.clone(), waypoint_position.clone());
        ship_position = (result.clone().0, result.clone().1);
        waypoint_position = (result.clone().2, result.clone().3);
    }

    return ship_position.0.abs() + ship_position.1.abs();
}

fn handle_instruction_part_two(instruction: String, ship_position: (isize, isize), waypoint_position: (isize, isize)) -> (isize, isize, isize, isize) {
    let command = instruction[..1].to_string();
    let value = instruction[1..].parse::<isize>().unwrap();
    let mut new_ship_position = (0,0);
    let mut new_waypoint_position = (0,0);

    if command == "N" {
        new_ship_position = ship_position;
        new_waypoint_position.0 = waypoint_position.0;
        new_waypoint_position.1 += waypoint_position.1 + value;
    }

    if command == "S" {
        new_ship_position = ship_position;
        new_waypoint_position.0 = waypoint_position.0;
        new_waypoint_position.1 += waypoint_position.1 - value;
    }

    if command == "E" {
        new_ship_position = ship_position;
        new_waypoint_position.0 += waypoint_position.0 + value;
        new_waypoint_position.1 = waypoint_position.1;
    }

    if command == "W" {
        new_ship_position = ship_position;
        new_waypoint_position.0 += waypoint_position.0 - value;
        new_waypoint_position.1 = waypoint_position.1;
    }

    if command == "F" {
        let previous_x_diff = waypoint_position.0 - ship_position.0;
        let previous_y_diff = waypoint_position.1 - ship_position.1;
        new_ship_position.0 += ship_position.0 + previous_x_diff * value;
        new_ship_position.1 += ship_position.1 + previous_y_diff * value;
        new_waypoint_position.0 += new_ship_position.0 + previous_x_diff;
        new_waypoint_position.1 += new_ship_position.1 + previous_y_diff;
    } 

    if command == "R" {
        new_ship_position = ship_position;

        if value == 90 {
            let result = rotate_waypoint_right_90(ship_position, waypoint_position);
            new_waypoint_position.0 += result.0;
            new_waypoint_position.1 += result.1;
        }
        if value == 180 {
            let result = rotate_waypoint_right_90(ship_position, rotate_waypoint_right_90(ship_position, waypoint_position));
            new_waypoint_position.0 += result.0;
            new_waypoint_position.1 += result.1;
        }
        if value == 270 {
            let result = rotate_waypoint_right_90(ship_position, rotate_waypoint_right_90(ship_position, rotate_waypoint_right_90(ship_position, waypoint_position)));
            new_waypoint_position.0 += result.0;
            new_waypoint_position.1 += result.1;
        }
    }

    if command == "L" {
        new_ship_position = ship_position;

        if value == 90 {
            let result = rotate_waypoint_left_90(ship_position, waypoint_position);
            new_waypoint_position.0 += result.0;
            new_waypoint_position.1 += result.1;            
        }
        if value == 180 {
            let result = rotate_waypoint_left_90(ship_position, rotate_waypoint_left_90(ship_position, waypoint_position));
            new_waypoint_position.0 += result.0;
            new_waypoint_position.1 += result.1;
        }
        if value == 270 {
            let result = rotate_waypoint_left_90(ship_position, rotate_waypoint_left_90(ship_position, rotate_waypoint_left_90(ship_position, waypoint_position)));
            new_waypoint_position.0 += result.0;
            new_waypoint_position.1 += result.1;
        }
    }

    return (new_ship_position.0, new_ship_position.1, new_waypoint_position.0, new_waypoint_position.1);
}

fn rotate_waypoint_left_90(ship_position: (isize, isize), waypoint_position: (isize, isize)) -> (isize, isize) {
    let x_direction = {
        if waypoint_position.0 - ship_position.0 < 0 { "W".to_string() } else { "E".to_string() }
    };
    let y_direction = {
        if waypoint_position.1 - ship_position.1 < 0 { "S".to_string() } else { "N".to_string() }
    };
    let previous_x_diff = (waypoint_position.0 - ship_position.0).abs();
    let previous_y_diff = (waypoint_position.1 - ship_position.1).abs();

    let new_waypoint_x = {
        if y_direction == "S" { ship_position.0 + previous_y_diff } else { ship_position.0 - previous_y_diff }
    };
    let new_waypoint_y = {
        if x_direction == "E" { ship_position.1 + previous_x_diff } else { ship_position.1 - previous_x_diff }
    };
    return (new_waypoint_x, new_waypoint_y);
}

fn rotate_waypoint_right_90(ship_position: (isize, isize), waypoint_position: (isize, isize)) -> (isize, isize) {
    let x_direction = {
        if waypoint_position.0 - ship_position.0 < 0 { "W".to_string() } else { "E".to_string() }
    };
    let y_direction = {
        if waypoint_position.1 - ship_position.1 < 0 { "S".to_string() } else { "N".to_string() }
    };
    let previous_x_diff = (waypoint_position.0 - ship_position.0).abs();
    let previous_y_diff = (waypoint_position.1 - ship_position.1).abs();
    let new_waypoint_x = {
        if y_direction == "S" { ship_position.0 - previous_y_diff } else { ship_position.0 + previous_y_diff }
    };
    let new_waypoint_y = {
        if x_direction == "E" { ship_position.1 - previous_x_diff } else { ship_position.1 + previous_x_diff }
    };
    return (new_waypoint_x, new_waypoint_y);
}

fn handle_instruction(instruction: String, direction: String, ship_position: (isize, isize)) -> (isize, isize, String) {
    let command = instruction[..1].to_string();
    let value = instruction[1..].parse::<isize>().unwrap();

    if command == "F" {
        let new_position = move_in_direction(direction.clone(), value, ship_position);
        return (new_position.0, new_position.1, direction);
    } 

    let mut new_direction = direction.clone();
    if command == "R" || command == "L" {
        new_direction = rotate_direction(direction.clone(), command.clone(), value);
    }
    
    let new_position = move_in_direction(command, value, ship_position);
    return (new_position.0, new_position.1, new_direction);
}

fn move_in_direction(direction: String, value: isize, position: (isize, isize)) -> (isize, isize) {
    let mut new_position = position.clone();
    if direction == "E" || direction == "W" {
        if direction == "E" {new_position.0 += value} else {new_position.0 -= value};
    }

    if direction == "N" || direction == "S" {
        if direction == "N" {new_position.1 += value} else {new_position.1 -= value};
    }

    return new_position;
}

fn rotate_direction(direction: String, command: String, value: isize) -> String {
    if value == 0 {
        return direction;
    }

    let directions = vec!["N","E","S","W"];
    let steps = if value == 90 {1} else if value == 180 {2} else if value == 270 {3} else {0};
    let mut index: isize = directions.iter().position(|&s| s.to_string() == direction).unwrap().try_into().unwrap();
    if command == "R" {
        index += steps;
        if index >= directions.len() as isize { index -= directions.len() as isize} else { index = index };
    }
    if command == "L" {
        index -= steps;
        if index < 0 { index += directions.len() as isize} else { index = index };
    }

    return directions[index as usize].to_string();
}

#[cfg(test)]
#[path = "tests/day_12_tests.rs"]
mod day_12_tests;