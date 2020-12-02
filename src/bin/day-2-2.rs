use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("src/bin/inputs/day-2-input.txt")?;
    let reader = BufReader::new(file);
    let mut counter = 0;
    for line in reader.lines() {
        let line = line?;
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

    println!("{}", counter);

    Ok(())
}

fn split_line<'a>(separator: &'a str, line: &'a str) -> Vec<&'a str> {
    return line.split(separator).collect::<Vec<&str>>();
}