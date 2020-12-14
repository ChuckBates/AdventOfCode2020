use std::time::Instant;
use std::collections::BTreeMap;
mod input;

fn main() {
    let now = Instant::now();

    // Part One
    let input_file_path_part_one = "src/bin/inputs/day_13_input_part_one.txt";
    let part_one_result = execute_part_one(input_file_path_part_one);
    println!("({} seconds) product of bus ID and time to wait: {}", now.elapsed().as_secs_f32(), part_one_result);

    // Part Two
    let input_file_path_part_one = "src/bin/inputs/day_13_input_part_two.txt";
    let part_two_result = execute_part_two(input_file_path_part_one);
    println!("({} seconds) earliest timestamp: {}", now.elapsed().as_secs_f32(), part_two_result);
}

fn execute_part_one(file_path: &str) -> usize {
    let lines = input::parse_strings(file_path);
    let time: usize = lines[0].parse::<usize>().unwrap();
    let mut next_bus_departures = vec![];
    for interval in lines[1].split(',') {
        if interval == "x" {
            continue;
        }
        let interval = interval.parse::<usize>().unwrap();
        next_bus_departures.push((get_bus_next_departure(time, interval), interval));
    }
    next_bus_departures.sort();
    return (next_bus_departures[0].0 - time) * next_bus_departures[0].1; 
}

fn execute_part_two(file_path: &str) -> usize {
    let lines = input::parse_strings(file_path);
    let mut bus_map = BTreeMap::new();
    let mut index = 0;
    let mut largest_interval = 0;
    for interval in lines[1].split(',') {
        if interval != "x" {
            let window = interval.parse::<usize>().unwrap();
            bus_map.insert(index, window);
            if window > largest_interval {
                largest_interval = window;
            }
        }
        index += 1;
    }

    let mut time = 0;
    let mut multiplier = 1;
    for (index, bus_interval) in bus_map {
        while (time + index) % bus_interval != 0 {
            time += multiplier;
        }
        multiplier *= bus_interval;
    }    

    return time;
}

fn get_bus_next_departure(time: usize, bus_interval: usize) -> usize {
    let mut result = time;
    loop {
        if result % bus_interval == 0 {
            break;
        }
        result += 1;
    }

    return result;
}

#[cfg(test)]
#[path = "tests/day_13_tests.rs"]
mod day_13_tests;