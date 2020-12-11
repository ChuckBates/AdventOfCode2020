use std::time::Instant;
mod input;

fn main() {
    let input_file_path = "src/bin/inputs/day_eleven_input.txt";
    let now = Instant::now();

    // Part One
    let part_one_result = execute_part_one(input_file_path);
    println!("({} occupied seats: {}", now.elapsed().as_secs_f32(), part_one_result);

    // Part Two
    let part_two_result = execute_part_two(input_file_path);
    println!("({} occupied seats: {}", now.elapsed().as_secs_f32(), part_two_result);
}

fn execute_part_one(file_path: &str) -> usize {
    let initial_grid = build_inital_grid(file_path);
    let final_grid = iterate_grid_until_stale(initial_grid, 4, true);   

    return count_occupied_states(final_grid);
}

fn execute_part_two(file_path: &str) -> usize {
    let initial_grid = build_inital_grid(file_path);
    let final_grid = iterate_grid_until_stale(initial_grid, 5, false);

    return count_occupied_states(final_grid);
}

fn build_inital_grid(file_path: &str) -> Vec<Vec<String>> {
    let lines = input::parse_strings(file_path);
    let mut grid = vec![];
    let size = lines[0].len() + 2;
    grid.push(vec![".".to_string(); size]);
    for line in lines {
        let mut characters = vec![];
        characters.push(".".to_string());
        for character in line.chars() {
            characters.push(character.to_string());
        }
        characters.push(".".to_string());
        grid.push(characters);
    }
    grid.push(vec![".".to_string(); size]);

    return grid;
}

fn iterate_grid_until_stale(grid: Vec<Vec<String>>, tolerance: usize, is_adjacent_only: bool) -> Vec<Vec<String>> {
    let x_max = grid.len();
    let y_max = grid[0].len();
    let prerimeter: Vec<(isize, isize)> = vec![
        (-1,-1),
        (-1,-0),
        (-1,1),
        (0,-1),
        (0,1),
        (1,-1),
        (1,0),
        (1,1)
    ];

    let mut next_state = grid.clone();
    let mut state = vec![];
    while !next_state.iter().eq(state.iter()) {
        state = next_state.clone();
        for x in 1..x_max {
            for y in 1..y_max {
                if state[x][y] == "." {
                    continue;
                }

                let mut adjacent_states = 0;
                for position in prerimeter.clone() {
                    let mut i = (x as isize + position.0) as usize;
                    let mut j = (y as isize + position.1) as usize;

                    if !is_adjacent_only {
                        while state[i][j] == "." && i < x_max - 1 && i > 0 && j < y_max -1 && j > 0 {
                            i = (i as isize + position.0) as usize;
                            j = (j as isize + position.1) as usize;
                        }
                    }

                    if state[i][j] == "#" {
                        adjacent_states += 1;
                    }
                }
                
                if state[x][y] == "#" && adjacent_states >= tolerance {
                    next_state[x][y] = "L".to_string();
                }
                if state[x][y] == "L" && adjacent_states <= 0 {
                    next_state[x][y] = "#".to_string();
                }
            }
        }
    }

    return state;
}

fn count_occupied_states(grid: Vec<Vec<String>>) -> usize {
    let mut counter = 0;
    for line in grid {
        counter += line.iter().filter(|&s| *s == "#").count();
    }

    return counter;
}

#[cfg(test)]
#[path = "tests/day_11_tests.rs"]
mod day_11_tests;