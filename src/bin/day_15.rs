use std::time::Instant;
mod input;

fn main() {
    let input_file_path = "src/bin/inputs/day_15_input.txt";
    let now = Instant::now();

    // Part One
    let part_one_result = execute(input_file_path, 2020);
    println!("({} seconds) final number spoken: {}", now.elapsed().as_secs_f32(), part_one_result);

    // Part Two
    let part_two_result = execute(input_file_path, 30000000);
    println!("({} seconds) final number spoken: {}", now.elapsed().as_secs_f32(), part_two_result);

}

fn execute(file_path: &str, limit: usize) -> usize {
    let lines = input::parse_strings(file_path);
    let mut spoken_numbers = vec![0; limit];
    let mut index = 1;
    let mut last_spoken = 0;
    for start_number in lines[0].split(",") {
        spoken_numbers[start_number.parse::<usize>().unwrap()] = index;
        index += 1;
    }

    while index < limit {
        let mut next = 0;
        if spoken_numbers[last_spoken] != 0 {
            next = index - spoken_numbers[last_spoken];
        }

        spoken_numbers[last_spoken] = index;
        last_spoken = next;

        index += 1;
    }

    return last_spoken;
}

#[cfg(test)]
#[path = "tests/day_15_tests.rs"]
mod day_15_tests;